use crate::ast::keywords::Keyword;
use crate::basic_ast::punctuation::Punctuation;
use crate::basic_ast::symbol::{BasicAbstractSyntaxTree, BasicSymbol, NameType};
use crate::processor::processor::ProcessorError;
use crate::processor::processor::ProcessorError::Syntax;

use std::path::{PathBuf};
use std::vec::IntoIter;


pub type PreProcessFunction = (
    String,
    Vec<(String, usize, String, usize)>,
    Option<String>,
    Vec<BasicSymbol>,
);

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum PreprocessSymbol {
    Struct(PathBuf, String, Vec<(String, usize, String, usize)>),
    Impl(PathBuf, String, Vec<(PreProcessFunction, usize)>),
    Fn(PathBuf, PreProcessFunction),
}

fn no_name_error(path: PathBuf, line: usize, kw: &str) -> ProcessorError {
    Syntax(path, line, format!("{kw} must be followed by a name"))
}

fn no_name_error_followed(path: PathBuf, line: usize, kw: &str, s: BasicSymbol) -> ProcessorError {
    Syntax(
        path,
        line,
        format!("{kw} must be followed by a name, not {}", s.instead_found()),
    )
}

fn bad_name(path: PathBuf, line: usize, kw: &str) -> ProcessorError {
    Syntax(
        path,
        line,
        format!("{kw} must be followed by a name that does not contain a '.'"),
    )
}

fn no_braces(path: PathBuf, line: usize, kw: &str) -> ProcessorError {
    Syntax(
        path,
        line,
        format!("{kw}'s name must be followed with braces ('{{')"),
    )
}

pub fn preprocess(
    ast: Vec<BasicAbstractSyntaxTree>,
) -> Result<Vec<(PreprocessSymbol, usize)>, ProcessorError> {
    let mut output = Vec::new();

    for (path, tree) in ast {
        let mut tree = tree.into_iter();
        loop {
            let next = tree.next();
            if next.is_none() {
                break;
            }
            let first_symbol = next.unwrap();

            match first_symbol {
                BasicSymbol::Keyword(keyword) => match keyword {
                    Keyword::Struct => {
                        output.push(parse_struct(path.clone(), &mut tree)?);
                    }
                    Keyword::Impl => {
                        output.push(parse_impl(path.clone(), &mut tree)?);
                    }
                    Keyword::Fn => {
                        output.push(parse_fn(path.clone(), &mut tree, 9999999)?);
                    }
                    _ => {}
                },
                BasicSymbol::AbstractSyntaxTree(_) => panic!(),
                symbol => {
                    return Err(Syntax(
                        path,
                        999999,
                        format!(
                            "expected 'struct', 'impl' or 'fn' but instead found {}",
                            symbol.instead_found()
                        ),
                    ))
                }
            }
        }
    }

    Ok(output)
}

fn parse_struct(
    path: PathBuf,
    tree: &mut IntoIter<BasicSymbol>
) -> Result<(PreprocessSymbol, usize), ProcessorError> {
    let name = tree
        .next()
        .ok_or(no_name_error(path.clone(), 999999, "struct"))?;
    let mut name = match name {
        BasicSymbol::Name(name) => name,
        _ => return Err(no_name_error_followed(path, 999999, "struct", name)),
    };

    if name.len() > 1 {
        return Err(bad_name(path, 999999, "struct"));
    }
    let name = name.remove(0);

    let contents =
        tree.next()
            .ok_or(no_braces(path.clone(), 999999, "struct"))?;
    let contents = match contents {
        BasicSymbol::BracedSection(contents) => contents,
        _ => return Err(no_braces(path, 999999, "struct")),
    };
    let mut contents = contents.into_iter();

    let mut attributes = Vec::new();
    let mut first = true;

    loop {
        let mut first_item = contents.next();
        if first_item.is_none() {
            break;
        }

        if !first {
            let tmp_first_item = first_item.unwrap();
            if !matches!(
                tmp_first_item,
                BasicSymbol::Punctuation(Punctuation::ListSeparator)
            ) {
                return Err(Syntax(
                    path,
                    999999,
                    "struct attributes must be ',' separated".to_string(),
                ));
            }
            first_item = contents.next();
            if first_item.is_none() {
                break;
            }
        }

        let attr_name = first_item.unwrap();
        let attr_name = match attr_name {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(Syntax(
                        path,
                        999999,
                        "struct attribute name cannot contain '.'".to_string(),
                    ));
                }
                name.remove(0)
            }
            _ => {
                return Err(Syntax(
                    path,
                    999999,
                    "expected name of attribute".to_string(),
                ))
            }
        };
        let colon = contents.next();
        if colon.is_none()
            || !matches!(
                colon.as_ref().unwrap(),
                BasicSymbol::Punctuation(Punctuation::Colon)
            )
        {
            return Err(Syntax(
                path,
                999999,
                "expected ':' after attribute name".to_string(),
            ));
        }
        let colon_line = 999999;

        let attr_type = contents.next();
        if attr_type.is_none() {
            return Err(Syntax(
                path,
                colon_line,
                "expected type after attribute name and ':'".to_string(),
            ));
        }
        let attr_type = attr_type.unwrap();
        let attr_type = match attr_type {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(Syntax(
                        path,
                        999999,
                        "attribute types cannot contain '.'".to_string(),
                    ));
                }
                name.remove(0)
            }
            _ => {
                return Err(Syntax(
                    path,
                    999999,
                    "expected attribute type after attribute name and ':'".to_string(),
                ))
            }
        };

        attributes.push((attr_name.0, 999999, attr_type.0, 999999));
        first = false;
    }

    Ok((PreprocessSymbol::Struct(path, name.0, attributes), 999999))
}

fn parse_impl(
    path: PathBuf,
    tree: &mut IntoIter<BasicSymbol>,
) -> Result<(PreprocessSymbol, usize), ProcessorError> {
    let name= tree
        .next()
        .ok_or(no_name_error(path.clone(), 999999, "impl"))?;
    let mut name = match name {
        BasicSymbol::Name(name) => name,
        _ => return Err(no_name_error_followed(path, 999999, "impl", name)),
    };

    if name.len() > 1 {
        return Err(bad_name(path, 999999, "impl"));
    }
    let name = name.remove(0);

    let contents =
        tree.next()
            .ok_or(no_braces(path.clone(), 999999, "impl"))?;
    let contents = match contents {
        BasicSymbol::BracedSection(contents) => contents,
        _ => return Err(no_braces(path, 999999, "impl")),
    };
    let mut contents = contents.into_iter();

    let mut functions = Vec::new();

    loop {
        let symbol = contents.next();
        if symbol.is_none() {
            break;
        }
        let symbol = symbol.unwrap();
        match symbol {
            BasicSymbol::Keyword(Keyword::Fn) => {
                let (function, fn_line) = parse_fn(path.clone(), &mut contents, 999999)?;
                let function = match function {
                    PreprocessSymbol::Fn(_, function) => function,
                    _ => panic!(),
                };
                functions.push((function, fn_line));
            }
            _ => {
                return Err(Syntax(
                    path,
                    999999,
                    "only function definitions (beginning with 'fn') allowed within impls"
                        .to_string(),
                ))
            }
        }
    }

    Ok((PreprocessSymbol::Impl(path, name.0, functions), 999999))
}

fn parse_fn(
    path: PathBuf,
    tree: &mut IntoIter<BasicSymbol>,
    main_line: usize,
) -> Result<(PreprocessSymbol, usize), ProcessorError> {
    let name = tree
        .next()
        .ok_or(no_name_error(path.clone(), main_line, "fn"))?;
    let mut name = match name {
        BasicSymbol::Name(name) => name,
        _ => return Err(no_name_error_followed(path, 999999, "fn", name)),
    };

    if name.len() > 1 {
        return Err(bad_name(path, 999999, "fn"));
    }
    let (name, _, name_type) = name.remove(0);

    let parameters = match name_type {
        NameType::Normal => return Err(Syntax(
            path.clone(),
            999999,
            "function name must be followed by brackets ('()')".to_string(),
        )),
        NameType::Function(arguments) => {
            arguments
        }
    };

    // let (arguments, arguments_line) = tree.next().ok_or()?;
    // let arguments = match arguments {
    //     BasicSymbol::BracketedSection(contents) => contents,
    //     _ => {
    //         return Err(Syntax(
    //             path,
    //             arguments_line,
    //             "function name must be followed by brackets ('()')".to_string(),
    //         ))
    //     }
    // };

    let mut parameters_processed = Vec::new();

    for parameter in parameters {
        let mut parameter = parameter.into_iter();

        let mut first_item = parameter.next();
        if first_item.is_none() {
            return Err(Syntax(
                path,
                999999, // TODO:
                "function parameters cannot have a trailing ','".to_string(),
            ));
        }

        let arg_name = first_item.unwrap();
        let arg_name = match arg_name {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(Syntax(
                        path,
                        999999, // TODO:
                        "function parameter name cannot contain '.'".to_string(),
                    ));
                }
                name.remove(0).0
            }
            _ => {
                return Err(Syntax(
                    path,
                    999999,
                    "expected name of parameter".to_string(),
                ))
            }
        };

        let colon = parameter.next();
        if colon.is_none()
            || !matches!(
                colon.as_ref().unwrap(),
                BasicSymbol::Punctuation(Punctuation::Colon)
            )
        {
            return Err(Syntax(
                path,
                999999, // TODO:
                "expected ':' after parameter name".to_string(),
            ));
        }

        let param_type = parameter.next();
        if param_type.is_none() {
            return Err(Syntax(
                path,
                999999, // TODO:
                "expected type after argument name and ':'".to_string(),
            ));
        }
        let param_type = param_type.unwrap();
        let param_type = match param_type {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(Syntax(
                        path,
                        999999, // TODO:
                        "parameter types cannot contain '.'".to_string(),
                    ));
                }
                name.remove(0).0
            }
            _ => {
                return Err(Syntax(
                    path,
                    999999, // TODO:
                    "expected parameter type after attribute name and ':'".to_string(),
                ))
            }
        };

        parameters_processed.push((arg_name, 999999, param_type, 999999)); // TODO:
    }

    let mut contents = tree.next().ok_or(Syntax(
        path.clone(),
        999999,
        "function arguments must be followed by braces ('{}')".to_string(),
    ))?;

    let return_type = if matches!(&contents, BasicSymbol::Punctuation(Punctuation::Tilda)) {
        match tree.next() {
            None => {
                return Err(Syntax(
                    path.clone(),
                    999999,
                    "'~' must be followed by a return type".to_string(),
                ))
            }
            Some(BasicSymbol::Name(mut name)) => {
                if name.len() > 1 {
                    return Err(Syntax(
                        path,
                        999999,
                        "function return type cannot contain '.'".to_string(),
                    ));
                }
                Some(name.remove(0))
            }
            _ => {
                return Err(Syntax(
                    path.clone(),
                    999999,
                    "'~' must be followed by a return type".to_string(),
                ))
            }
        }
    } else {
        None
    };

    if return_type.is_some() {
        contents = tree.next().ok_or(Syntax(
            path.clone(),
            999999,
            "function return type must be followed by braces ('{}')".to_string(),
        ))?;
    }

    let contents =
        match contents {
            BasicSymbol::BracedSection(contents) => contents,
            _ => return Err(Syntax(
                path.clone(),
                999999,
                "function arguments (and optional return type) must be followed by braces ('{}')"
                    .to_string(),
            )),
        };

    Ok((
        PreprocessSymbol::Fn(path, (name, parameters_processed, return_type.and_then(|x| Some(x.0)), contents)),
        main_line,
    ))
}
