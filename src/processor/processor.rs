use crate::basic_ast::symbol::{BasicAbstractSyntaxTree};
use crate::processor::preprocess::preprocess;
use crate::processor::type_builder::build_types;
use std::path::PathBuf;
use thiserror::Error;
use crate::processor::function_processor::{Function, process_functions};

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("syntax error in file {0}:{1} - {2}")]
    Syntax(PathBuf, usize, String),
    #[error("type '{2}' in file {0}:{1} not found")]
    TypeNotFound(PathBuf, usize, String),
    #[error("type '{2}' in file {0}:{1} also defined in file {3}:{4}")]
    TypeRedefinition(PathBuf, usize, String, PathBuf, usize),
    #[error("type '{2}' has an infinite size")]
    CircularType(PathBuf, usize, String),
    #[error("TODO: BadImplType")]
    BadImplType(PathBuf), // TODO
    #[error("TODO: No main")]
    NoMainFunction,
    #[error("TODO: Function redefinition")]
    FunctionRedefinition,
    #[error("TODO: Bad main function - {0}")]
    BadMainFunction(String),
    #[error("TODO: variable already defined - {0}")]
    VariableAlreadyDefined(String)
}

pub fn process(ast: Vec<BasicAbstractSyntaxTree>) -> Result<Vec<Box<dyn Function>>, ProcessorError> {
    let pre_ast = preprocess(ast)?;
    println!("Preprocessing Result:\n{:?}", pre_ast);
    let (type_table, function_names, typed_functions) = build_types(pre_ast)?;
    println!("Typed functions:\n{:?}", typed_functions);
    process_functions(function_names, typed_functions, type_table)
}
