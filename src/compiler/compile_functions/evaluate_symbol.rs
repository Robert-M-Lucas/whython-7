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
        BasicSymbol::Literal(literal) => Some(instantiate_literal::instantiate_variable(
            Left((literal, &symbol.1)),
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
                Left(new_variable) => {
                    if let Some(return_into) = return_into {
                        if return_into.1 != new_variable.1 {
                            return Err(ProcessorError::BadEvaluatedType(
                                symbol.1.clone(),
                                name_handler.type_table().get_type(return_into.1).unwrap().get_name().to_string(),
                                name_handler.type_table().get_type(new_variable.1).unwrap().get_name().to_string()
                            ));
                        }

                        lines.push(
                            Line::Copy(new_variable.0, return_into.0, name_handler.type_table().get_type(return_into.1).unwrap().get_size(name_handler.type_table(), None)?)
                        );

                        Some(return_into)
                    }
                    else {
                        Some(new_variable)
                    }
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
