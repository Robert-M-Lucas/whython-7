use crate::basic_ast::symbol::{BasicAbstractSyntaxTree};
use crate::processor::preprocess::preprocess;
use crate::processor::type_builder::build_type_table;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("syntax error in file {0}:{1} - {2}")]
    Syntax(PathBuf, usize, String),
    #[error("type '{2}' in file {0}:{1} not found")]
    TypeNotFound(PathBuf, usize, String),
    #[error("type '{2}' in file {0}:{1} also defined in file {3}:{4}")]
    TypeRedefinition(PathBuf, usize, String, PathBuf, usize),
    #[error("type '{2}' has an infinite size")]
    CircularType(PathBuf, usize, String)
}

pub fn process(ast: Vec<BasicAbstractSyntaxTree>) -> Result<(), ProcessorError> {
    let pre_ast = preprocess(ast)?;
    println!("Preprocessing Result:\n{:?}", pre_ast);
    let (_type_table, _pre_ast) = build_type_table(pre_ast)?;
    Ok(())
}
