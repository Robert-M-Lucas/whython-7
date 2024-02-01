use std::path::PathBuf;
use thiserror::Error;
use crate::basic_ast::symbol::{BasicAbstractSyntaxTree, BasicSymbol};
use crate::processor::preprocess::preprocess;


#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("syntax error in file {0}:{1} - {2}")]
    SyntaxError(PathBuf, usize, String),
}

pub fn process(ast: Vec<BasicAbstractSyntaxTree>) -> Result<(), ProcessorError> {
    let pre_ast = preprocess(ast)?;
    Ok(())
}