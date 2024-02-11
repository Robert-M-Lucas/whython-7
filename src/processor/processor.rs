use crate::basic_ast::symbol::{BasicAbstractSyntaxTree, BasicSymbol};
use crate::processor::preprocess::preprocess;
use crate::processor::type_builder::build_types;
use std::path::PathBuf;
use thiserror::Error;
use crate::compiler::compile_functions::{Function, compile_functions};

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("syntax error in file {0}:{1} - {2}")]
    Syntax(PathBuf, usize, String),
    #[error("type '{2}' in file {0}:{1} not found")]
    TypeNotFound(PathBuf, usize, String),
    #[error("TODO: name '{0}' not found")] // TODO:
    NameNotFound(String),
    #[error("type '{2}' in file {0}:{1} also defined in file {3}:{4}")]
    TypeRedefinition(PathBuf, usize, String, PathBuf, usize),
    #[error("type '{2}' has an infinite size")]
    CircularType(PathBuf, usize, String),
    #[error("TODO: BadImplType")]
    BadImplType(PathBuf), // TODO:
    #[error("TODO: No main")]
    NoMainFunction,
    #[error("TODO: Function redefinition")]
    FunctionRedefinition,
    #[error("TODO: Bad main function - {0}")]
    BadMainFunction(String),
    #[error("TODO: variable already defined - {0}")]
    VariableAlreadyDefined(String),
    #[error("TODO: Bad keyword")]
    BadKeyword,
    #[error("TODO: Expected semicolon")]
    ExpectedSemicolon,
    #[error("TODO: Empty brackets")]
    EmptyBrackets,
    #[error("TODO: Bad item in evaluated section")]
    BadItemInEvaluation,
    #[error("TODO: Expected operator and operand")]
    ExpectedOperatorOperand,
    #[error("TODO: Bad operator position")]
    BadOperatorPosition,
    #[error("TODO: Bad operator function (did you override an operator?)")]
    BadOperatorFunction,
    #[error("TODO: Standalone type")]
    StandaloneType,
    #[error("TODO: Standalone operator")]
    StandaloneOperator,
    #[error("TODO: Doesn't evaluate")]
    DoesntEvaluate,
    #[error("TODO: Bad arg type")]
    BadArgType,
    #[error("TODO: Bad arg count")]
    BadArgCount,
    #[error("TODO: Bad evaluable layout - Expected `value`, `prefix-operator value`, or `value postfix-operator other-value`")]
    BadEvaluableLayout,
    #[error("TODO: Unexpected symbol")]
    UnexpectedSymbol(BasicSymbol),
    #[error("Placeholder")]
    Placeholder
}

pub fn process(ast: Vec<BasicAbstractSyntaxTree>) -> Result<Vec<Box<dyn Function>>, ProcessorError> {
    let pre_ast = preprocess(ast)?;
    // println!("Preprocessing Result:\n{:?}", pre_ast);
    let (type_table, function_names, typed_functions) = build_types(pre_ast)?;
    // println!("Typed functions:\n{:?}", typed_functions);
    compile_functions(function_names, typed_functions, type_table)
}
