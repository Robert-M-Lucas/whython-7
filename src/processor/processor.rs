use crate::ast::operators::Operator;
use crate::basic_ast::symbol::{BasicAbstractSyntaxTree};
use crate::compiler::compile_functions::{compile_functions, Function};
use crate::parser::line_info::LineInfo;
use crate::processor::preprocess::preprocess;
use crate::processor::type_builder::build_types;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("Error: Expected name after 'struct'\n{0}")]
    StructNoName(LineInfo),
    #[error("Error: Tried to define a name with multiple parts\n{0}")]
    MultipartNameDef(LineInfo),
    #[error("Error: Expected braces after struct name\n{0}")]
    StructNoBraces(LineInfo),
    #[error("Error: Struct attributes must be ',' separated\n{0}")]
    StructNoAttrSeparator(LineInfo),
    #[error("Error: Expected struct attribute name\n{0}")]
    StructExpectedAttributeName(LineInfo),
    #[error("Error: Expected `: [TYPE]` after name to define type\n{0}")]
    NameTypeNotDefined(LineInfo),
    #[error("Error: Expected 'struct', 'impl' or 'fn' at top level\n{0}")]
    BadTopLevelSymbol(LineInfo),
    #[error("Error: Expected name after 'impl'\n{0}")]
    ImplNoName(LineInfo),
    #[error("Error: Expected braces after impl name\n{0}")]
    ImplNoBraces(LineInfo),
    #[error("Error: Only function definitions are allowed within impls\n{0}")]
    ImplNonFnContent(LineInfo),
    #[error("Error: Expected name after 'fn'\n{0}")]
    FnNoName(LineInfo),
    #[error("Error: Function parameters cannot have a trailing ','\n{0}")]
    FnParamsTrailingComma(LineInfo),
    #[error("Error: Expected parameter name \n{0}")]
    FnExpectedParameterName(LineInfo),
    #[error("Error: Expected '~ [RETURN TYPE]' or braces after function parameters\n{0}")]
    FnNoBracesOrReturn(LineInfo),
    #[error("Error: Expected braces after function parameters\n{0}")]
    FnNoBraces(LineInfo),
    #[error("Error: Expected function return type after '~'\n{0}")]
    FnExpectedReturnType(LineInfo),
    #[error("Error: 'self' can only be used as the first parameter in an impl function with no type specifier\n{0}")]
    FnBadSelf(LineInfo),
    #[error("Error: Parameter name '{1}' already in use\n{0}")]
    ParameterNameInUse(LineInfo, String),
    #[error("[TODO] Error: Tried to use a type name with multiple parts\n{0}")]
    MultipartTypeName(LineInfo),
    #[error("Error: Type '{1}' not found\n{0}")]
    TypeNotFound(LineInfo, String),
    #[error("Error: Name '{1}' not found\n{0}")] // TODO:
    NameNotFound(LineInfo, String),
    // #[error("Error: Type '{1}' defined...\n{0}{2}")]
    // TypeRedefinition(LineInfo, String, LineInfo),
    #[error("Error: Type '{1}' has an infinite size [{2}]\n{0}")]
    CircularType(LineInfo, String, String),
    #[error("Error: Impl type not found\n{0}")]
    BadImplType(LineInfo),
    #[error("Error: No main function found")]
    NoMainFunction,
    #[error("Error: Main function cannot have parameters")]
    MainFunctionParams, // TODO
    #[error("Error: Main function must return 'int'")]
    MainFunctionBadReturn, // TODO
    // #[error("Error: Expected semicolon\n{0}")]
    // ExpectedSemicolon(LineInfo),
    #[error("Error: Bad operator position for '{1}'\n{0}")]
    BadOperatorPosition(LineInfo, Operator),
    #[error("Error: Standalone type\n{0}")]
    StandaloneType(LineInfo),
    // #[error("Error: Standalone operator\n{0}")]
    // StandaloneOperator(LineInfo),
    #[error("Error: This must evaluate to a value but doesn't\n{0}")]
    DoesntEvaluate(LineInfo),
    #[error("Error: Bad argument type for function - expected '{1}', found '{2}'. Called:\n{0}\nDefined:\n{3}")]
    BadArgType(LineInfo, String, String, LineInfo),
    #[error("Error: Wrong amount of arguments for function - expected {1}, found {2} (including automatic self passing where applicable). Called:\n{0}\nDefined:\n{3}")]
    BadArgCount(LineInfo, usize, usize, LineInfo),
    #[error(
        "Error: Functions with a return type must have a return statement as their last line\n{0}"
    )]
    NoReturnStatement(LineInfo),
    #[error("Error: You can only assign to names\n{0}")]
    NonNameAssignment(LineInfo),
    #[error("Error: Assignment operator must have value on RHS\n{0}")]
    NoAssignmentRHS(LineInfo),
    #[error("Error: Can't return nothing from a function with a return type\n{0}")]
    NoneReturnOnTypedFunction(LineInfo),
    #[error("Error: Can't return a value from a function with no return type\n{0}")]
    TypeReturnOnVoidFunction(LineInfo),
    #[error("Error: Returned type '{2}' doesn't match function return type '{1}\n{0}")]
    BadReturnType(LineInfo, String, String),
    #[error("Error: Can only assign to variables")]
    AssignToNonVariable(LineInfo),
    #[error("Error: 'let' must be followed by a variable name")]
    LetNoName(LineInfo),
    #[error("Error: `let [NAME]: [TYPE]` must be followed by `= [VALUE]`")]
    LetNoValue(LineInfo),
    #[error("Error: `while` must be followed by brackets containing the condition\n{0}")]
    WhileNoBrackets(LineInfo),
    #[error("Error: Condition must evaluate to boolean (not '{1}')\n{0}")]
    BadConditionType(LineInfo, String),
    #[error("Error: While condition must be followed by braces containing the contents of the loop\n{0}")]
    WhileNoBraces(LineInfo),
    #[error("Error: While contents must be followed by semicolon\n{0}")]
    WhileMoreAfterBraces(LineInfo),
    #[error("Error: Evaluable layout must be `[VALUE]`, `[PREFIX OPERATOR] [VALUE]`, or `[VALUE] [POSTFIX OPERATOR] [OTHER VALUE]`\n\0")]
    BadEvaluableLayout(LineInfo),
    #[error("Error: Expected evaluation to type '{1}' but found '{2}'\n{0}")]
    BadEvaluatedType(LineInfo, String, String),
    #[error("Error: Operator function '{1}' not found for type '{2}'\n{0}")]
    SingleOpFunctionNotFound(LineInfo, String, String),
    #[error("Error: Operator function '{1}' not found for type '{2}' (LHS) and '{3}' (RHS)\n{0}")]
    OpFunctionNotFound(LineInfo, String, String, String),
    #[error("Error: Nothing can follow a 'break'\n{0}")]
    BreakLineNotEmpty(LineInfo),
    #[error("Error: Nothing to break out of\n{0}")]
    NothingToBreak(LineInfo),
    #[error("Error: `elif` and `else` can only follow `if`\n{0}")]
    RawElifElse(LineInfo),
    #[error("Error: `if` and `elif` must be followed by brackets containing the condition\n{0}")]
    IfElifNoBrackets(LineInfo),
    #[error("Error: `if` and `elif` conditions, or `else` on its own must be followed by braces containing code to be executed conditionally\n{0}")]
    IfElifElseNoBraces(LineInfo),
    #[error("Error: An if/elif/else chain must be followed by semicolon\n{0}")]
    ElseMoreAfterBraces(LineInfo),
    #[error("Error: Can't have anything after an else in an if/elif/else\n{0}")]
    IfElifAfterElse(LineInfo),
    #[error("TODO: Placeholder")]
    Placeholder2,
}

pub fn process(
    ast: Vec<BasicAbstractSyntaxTree>,
) -> Result<Vec<Box<dyn Function>>, ProcessorError> {
    let pre_ast = preprocess(ast)?;
    // println!("Preprocessing Result:\n{:?}", pre_ast);
    let (type_table, function_names, typed_functions) = build_types(pre_ast)?;
    // println!("Typed functions:\n{:?}", typed_functions);
    compile_functions(function_names, typed_functions, type_table)
}
