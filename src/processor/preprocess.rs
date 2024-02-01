use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::ast::keywords::Keyword;
use crate::basic_ast::symbol::{BasicAbstractSyntaxTree, BasicSymbol};
use crate::processor::processor::ProcessorError;
use crate::processor::processor::ProcessorError::SyntaxError;

pub type PreProcessFunction = (String, Vec<BasicSymbol>);

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum PreprocessSymbol {
    Struct(PathBuf, String, Vec<(String, String)>),
    Impl(PathBuf, String, Vec<PreProcessFunction>),
    Function(PathBuf, PreProcessFunction)
}

pub fn preprocess(ast: Vec<BasicAbstractSyntaxTree>) -> Result<Vec<(PathBuf, Vec<(PreprocessSymbol, usize)>)>, ProcessorError> {
    let mut output = Vec::new();

    fn no_name_error(path: PathBuf, line: usize, kw: &str) -> ProcessorError {
        SyntaxError(path, line, format!("{kw} must be followed by a name"))
    }

    fn no_name_error_followed(path: PathBuf, line: usize, kw: &str, s: BasicSymbol) -> ProcessorError {
        SyntaxError(path, line, format!("{kw} must be followed by a name, not {}", s.instead_found()))
    }

    fn bad_name(path: PathBuf, line: usize, kw: &str) -> ProcessorError {
        SyntaxError(path, line, format!("{kw} must be followed by a name that does not contain a '.'"))
    }

    for (path, tree) in ast {
        output.push((path.clone(), Vec::new()));

        let mut tree = tree.into_iter();
        loop {
            let next = tree.next();
            if next.is_none() { break; }
            let (first_symbol, line) = next.unwrap();

            match first_symbol {
                BasicSymbol::Keyword(keyword) => {
                    match keyword {
                        Keyword::Struct => {
                            let following = tree.next().ok_or(no_name_error(path.clone(), line, "struct"))?;
                            if let BasicSymbol::Name(name) = following.0 {
                                if name.len() > 1 { return Err(bad_name(path, following.1, "struct")); }

                            }
                            else {
                                return Err(no_name_error_followed(path, following.1, "struct", following.0));
                            }
                        }
                        Keyword::Impl => {
                            let following = tree.next().ok_or(no_name_error(path.clone(), line, "impl"))?;
                            if let BasicSymbol::Name(name) = following.0 {
                                if name.len() > 1 { return Err(bad_name(path, following.1, "impl")); }

                            }
                            else {
                                return Err(no_name_error_followed(path, following.1, "impl", following.0));
                            }
                        }
                        Keyword::Fn => {
                            let following = tree.next().ok_or(no_name_error(path.clone(), line, "fn"))?;
                            if let BasicSymbol::Name(name) = following.0 {
                                if name.len() > 1 { return Err(bad_name(path, following.1, "fn")); }

                            }
                            else {
                                return Err(no_name_error_followed(path, following.1, "fn", following.0));
                            }
                        }
                        _ => {}
                    }
                }
                BasicSymbol::AbstractSyntaxTree(_) => panic!(),
                symbol => return Err(SyntaxError(path, line, format!("expected 'struct', 'impl' or 'fn' but instead found {}", symbol.instead_found())))
            }
        }

    }

    Ok(output)
}