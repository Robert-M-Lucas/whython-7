use std::io;
use std::path::{Path, PathBuf};
use std::vec::IntoIter;
use thiserror::Error;
use crate::ast::keywords::Keyword;
use crate::basic_ast::punctuation::Punctuation;
use crate::basic_ast::symbol::{BasicAbstractSyntaxTree, BasicSymbol};
use crate::processor::processor::ProcessorError;
use crate::processor::processor::ProcessorError::SyntaxError;

pub type PreProcessFunction = (String, Vec<(String, usize, String, usize)>, Option<String>, Vec<(BasicSymbol, usize)>);

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum PreprocessSymbol {
    Struct(PathBuf, String, Vec<(String, usize, String, usize)>),
    Impl(PathBuf, String, Vec<(PreProcessFunction, usize)>),
    Function(PathBuf, PreProcessFunction)
}

fn no_name_error(path: PathBuf, line: usize, kw: &str) -> ProcessorError {
    SyntaxError(path, line, format!("{kw} must be followed by a name"))
}

fn no_name_error_followed(path: PathBuf, line: usize, kw: &str, s: BasicSymbol) -> ProcessorError {
    SyntaxError(path, line, format!("{kw} must be followed by a name, not {}", s.instead_found()))
}

fn bad_name(path: PathBuf, line: usize, kw: &str) -> ProcessorError {
    SyntaxError(path, line, format!("{kw} must be followed by a name that does not contain a '.'"))
}

fn no_braces(path: PathBuf, line: usize, kw: &str) -> ProcessorError {
    SyntaxError(path, line, format!("{kw}'s name must be followed with braces ('{{')"))
}

pub fn preprocess(ast: Vec<BasicAbstractSyntaxTree>) -> Result<Vec<(PreprocessSymbol, usize)>, ProcessorError> {
    let mut output = Vec::new();

    for (path, tree) in ast {

        let mut tree = tree.into_iter();
        loop {
            let next = tree.next();
            if next.is_none() { break; }
            let (first_symbol, main_line) = next.unwrap();

            match first_symbol {
                BasicSymbol::Keyword(keyword) => {
                    match keyword {
                        Keyword::Struct => {
                            output.push(parse_struct(path.clone(), &mut tree, main_line)?);
                        }
                        Keyword::Impl => {
                            output.push(parse_impl(path.clone(), &mut tree, main_line)?);
                        }
                        Keyword::Fn => {
                            output.push(parse_fn(path.clone(), &mut tree, main_line)?);
                        }
                        _ => {}
                    }
                }
                BasicSymbol::AbstractSyntaxTree(_) => panic!(),
                symbol => return Err(SyntaxError(path, main_line, format!("expected 'struct', 'impl' or 'fn' but instead found {}", symbol.instead_found())))
            }
        }

    }

    Ok(output)
}

fn parse_struct(path: PathBuf, tree: &mut IntoIter<(BasicSymbol, usize)>, main_line: usize) -> Result<(PreprocessSymbol, usize), ProcessorError> {
    let (name, name_line) = tree.next().ok_or(no_name_error(path.clone(), main_line, "struct"))?;
    let mut name = match name {
        BasicSymbol::Name(name) => name,
        _ => return Err(no_name_error_followed(path, name_line, "struct", name))
    };

    if name.len() > 1 { return Err(bad_name(path, name_line, "struct")); }
    let name = name.remove(0);

    let (contents, contents_line) = tree.next().ok_or(no_braces(path.clone(), name_line, "struct"))?;
    let contents = match contents {
        BasicSymbol::BracedSection(contents) => contents,
        _ => return Err(no_braces(path, contents_line, "struct"))
    };
    let mut contents = contents.into_iter();

    let mut attributes = Vec::new();
    let mut first = true;

    loop {
        let mut first_item = contents.next();
        if first_item.is_none() { break; }

        if !first {
            let (tmp_first_item, first_item_line) = first_item.unwrap();
            if !matches!(tmp_first_item, BasicSymbol::Punctuation(Punctuation::ListSeparator)) {
                return Err(SyntaxError(path, first_item_line, "struct attributes must be ',' separated".to_string()))
            }
            first_item = contents.next();
            if first_item.is_none() { break; }
        }

        let (attr_name, attr_line) = first_item.unwrap();
        let attr_name = match attr_name {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(SyntaxError(path, attr_line, "struct attribute name cannot contain '.'".to_string()))
                }
                name.remove(0)
            }
            _ => return Err(SyntaxError(path, attr_line, "expected name of attribute".to_string()))
        };
        let colon = contents.next();
        if colon.is_none() || !matches!(colon.as_ref().unwrap().0, BasicSymbol::Punctuation(Punctuation::Colon)) {
            return Err(SyntaxError(path, attr_line, "expected ':' after attribute name".to_string()))
        }
        let colon_line = colon.unwrap().1;

        let attr_type = contents.next();
        if attr_type.is_none() {
            return Err(SyntaxError(path, colon_line, "expected type after attribute name and ':'".to_string()))
        }
        let (attr_type, attr_type_line) = attr_type.unwrap();
        let attr_type = match attr_type {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(SyntaxError(path, contents_line, "attribute types cannot contain '.'".to_string()))
                }
                name.remove(0)
            }
            _ => return Err(SyntaxError(path, contents_line, "expected attribute type after attribute name and ':'".to_string()))
        };

        attributes.push((attr_name, attr_line, attr_type, attr_type_line));
        first = false;
    }

    Ok((PreprocessSymbol::Struct(path, name, attributes), main_line))
}

fn parse_impl(path: PathBuf, tree: &mut IntoIter<(BasicSymbol, usize)>, main_line: usize) -> Result<(PreprocessSymbol, usize), ProcessorError> {
    let (name, name_line) = tree.next().ok_or(no_name_error(path.clone(), main_line, "impl"))?;
    let mut name = match name {
        BasicSymbol::Name(name) => name,
        _ => return Err(no_name_error_followed(path, name_line, "impl", name))
    };

    if name.len() > 1 { return Err(bad_name(path, name_line, "impl")); }
    let name = name.remove(0);

    let (contents, contents_line) = tree.next().ok_or(no_braces(path.clone(), name_line, "impl"))?;
    let contents = match contents {
        BasicSymbol::BracedSection(contents) => contents,
        _ => return Err(no_braces(path, contents_line, "impl"))
    };
    let mut contents = contents.into_iter();

    let mut functions = Vec::new();

    loop {
        let symbol = contents.next();
        if symbol.is_none() { break; }
        let (symbol, symbol_line) = symbol.unwrap();
        match symbol {
            BasicSymbol::Keyword(Keyword::Fn) => {
                let (function, fn_line) = parse_fn(path.clone(), &mut contents, symbol_line)?;
                let function = match function {
                    PreprocessSymbol::Function(_, function) => function,
                    _ => panic!()
                };
                functions.push((function, fn_line));
            }
            _ => return Err(SyntaxError(path, symbol_line, "only function definitions (beginning with 'fn') allowed within impls".to_string()))
        }
    }

    Ok((PreprocessSymbol::Impl(path, name, functions), main_line))
}

fn parse_fn(path: PathBuf, tree: &mut IntoIter<(BasicSymbol, usize)>, main_line: usize) -> Result<(PreprocessSymbol, usize), ProcessorError> {
    let (name, name_line) = tree.next().ok_or(no_name_error(path.clone(), main_line, "fn"))?;
    let mut name = match name {
        BasicSymbol::Name(name) => name,
        _ => return Err(no_name_error_followed(path, name_line, "fn", name))
    };

    if name.len() > 1 { return Err(bad_name(path, name_line, "fn")); }
    let name = name.remove(0);

    let (arguments, arguments_line) = tree.next().ok_or(
        SyntaxError(path.clone(), name_line, "function name must be followed by brackets ('()')".to_string())
    )?;
    let arguments = match arguments {
        BasicSymbol::BracketedSection(contents) => contents,
        _ => return Err(SyntaxError(path, arguments_line, "function name must be followed by brackets ('()')".to_string()))
    };
    let mut arguments = arguments.into_iter();

    let mut arguments_processed = Vec::new();
    let mut first = true;

    loop {
        let mut first_item = arguments.next();
        if first_item.is_none() { break; }

        if !first {
            let (tmp_first_item, first_item_line) = first_item.unwrap();
            if !matches!(tmp_first_item, BasicSymbol::Punctuation(Punctuation::ListSeparator)) {
                return Err(SyntaxError(path, first_item_line, "function arguments must be ',' separated".to_string()))
            }
            first_item = arguments.next();
            if first_item.is_none() { break; }
        }

        let (arg_name, arg_line) = first_item.unwrap();
        let arg_name = match arg_name {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(SyntaxError(path, arg_line, "function argument name cannot contain '.'".to_string()))
                }
                name.remove(0)
            }
            _ => return Err(SyntaxError(path, arg_line, "expected name of argument".to_string()))
        };
        let colon = arguments.next();
        if colon.is_none() || !matches!(colon.as_ref().unwrap().0, BasicSymbol::Punctuation(Punctuation::Colon)) {
            return Err(SyntaxError(path, arg_line, "expected ':' after argument name".to_string()))
        }
        let colon_line = colon.unwrap().1;

        let arg_type = arguments.next();
        if arg_type.is_none() {
            return Err(SyntaxError(path, colon_line, "expected type after argument name and ':'".to_string()))
        }
        let (arg_type, arg_type_line) = arg_type.unwrap();
        let attr_type = match arg_type {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(SyntaxError(path, arg_type_line, "attribute types cannot contain '.'".to_string()))
                }
                name.remove(0)
            }
            _ => return Err(SyntaxError(path, arg_type_line, "expected attribute type after attribute name and ':'".to_string()))
        };

        arguments_processed.push((arg_name, arg_line, attr_type, arg_type_line));
        first = false;
    }

    let (mut contents, mut contents_line) = tree.next().ok_or(
        SyntaxError(path.clone(), name_line, "function arguments must be followed by braces ('{}')".to_string())
    )?;

    let return_type = if matches!(&contents, BasicSymbol::Punctuation(Punctuation::Tilda)) {
        match tree.next() {
            None => return Err(SyntaxError(path.clone(), contents_line, "'~' must be followed by a return type".to_string())),
            Some((BasicSymbol::Name(mut name), name_line)) => {
                if name.len() > 1 { return Err(SyntaxError(path, name_line, "function return type cannot contain '.'".to_string())) }
                Some(name.remove(0))
            }
            _ => return Err(SyntaxError(path.clone(), contents_line, "'~' must be followed by a return type".to_string()))
        }
    } else {
        None
    };

    if return_type.is_some() {
        (contents, contents_line) = tree.next().ok_or(
            SyntaxError(path.clone(), name_line, "function return type must be followed by braces ('{}')".to_string())
        )?;
    }

    let contents = match contents {
        BasicSymbol::BracedSection(contents) => contents,
        _ => return Err(SyntaxError(path.clone(), contents_line, "function arguments (and optional return type) must be followed by braces ('{}')".to_string()))
    };

    Ok((PreprocessSymbol::Function(path, (name, arguments_processed, return_type, contents)), main_line))
}