use crate::basic_ast::symbol::BasicSymbol;
use crate::compiler::compile_functions::{
    call_function, evaluate, instantiate_literal, FunctionHolder, Line, NameHandler,
};
use crate::parser::line_info::LineInfo;
use crate::processor::processor::ProcessorError;
use either::{Left, Right};

pub fn evaluate_symbol(
    symbol: &(BasicSymbol, LineInfo),
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<Option<(isize, isize)>, ProcessorError> {
    // println!("{:?}", symbol);
    Ok(match &symbol.0 {
        BasicSymbol::AbstractSyntaxTree(_) => panic!(),
        BasicSymbol::Operator(_) => {
            return Err(ProcessorError::BadEvaluableLayout(symbol.1.clone()))
        }
        BasicSymbol::Literal(literal) => Some(instantiate_literal::instantiate_literal(
            Left(literal),
            lines,
            name_handler,
            function_holder,
            return_into,
        )?),
        BasicSymbol::BracketedSection(inner) => {
            evaluate::evaluate(inner, lines, name_handler, function_holder, return_into)?
        }
        BasicSymbol::Name(name) => {
            // println!("{:?}", name);
            match name_handler.resolve_name(function_holder, name, &symbol.1)? {
                Left(variable) => {
                    // println!("{:?}", variable);
                    Some(variable)
                }
                Right((function, default_args, args)) => call_function::call_function(
                    function,
                    &symbol.1,
                    default_args,
                    args,
                    lines,
                    name_handler,
                    function_holder,
                    return_into,
                )?,
            }
        }
        _other => return Err(ProcessorError::BadEvaluableLayout(symbol.1.clone())),
    })
}
