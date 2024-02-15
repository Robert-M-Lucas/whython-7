use crate::ast::operators::Operator;
use crate::basic_ast::symbol::BasicSymbol;
use crate::compiler::compile_functions::{instantiate_literal, FunctionHolder, Line, NameHandler};
use crate::parser::line_info::LineInfo;
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{Type, TypedFunction};
use either::Right;

pub fn evaluate_operator(symbol: &(BasicSymbol, LineInfo)) -> Result<&Operator, ProcessorError> {
    match &symbol.0 {
        BasicSymbol::Operator(operator) => Ok(operator),
        _ => Err(ProcessorError::BadEvaluableLayout(symbol.1.clone())),
    }
}

pub fn evaluate_operation(
    lhs: (isize, isize),
    op: (&Operator, &LineInfo),
    rhs: Option<(isize, isize)>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<Option<(isize, isize)>, ProcessorError> {
    Ok(Some(match &op.0 {
        Operator::Not => {
            let func = function_holder.get_function(Some(lhs.1), "not").ok_or(
                ProcessorError::SingleOpFunctionNotFound(
                    op.1.clone(),
                    "not".to_string(),
                    name_handler
                        .type_table
                        .get_type(lhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                ),
            )?;
            name_handler.use_function(func);
            let func_args = func.get_args();
            let func_id = func.get_id();
            if func_args.len() != 1 {
                return Err(ProcessorError::SingleOpFunctionNotFound(
                    op.1.clone(),
                    "not".to_string(),
                    name_handler
                        .type_table
                        .get_type(lhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                ));
            }
            let output = if let Some(return_into) = return_into {
                return_into
            } else {
                instantiate_literal::instantiate_literal(
                    Right(
                        func.get_return_type()
                            .ok_or(ProcessorError::SingleOpFunctionNotFound(
                                op.1.clone(),
                                "not".to_string(),
                                name_handler
                                    .type_table
                                    .get_type(lhs.1)
                                    .unwrap()
                                    .get_name()
                                    .to_string(),
                            ))?,
                    ),
                    lines,
                    name_handler,
                    function_holder,
                    None,
                )?
            };
            let func = function_holder.functions().get(&func_id).unwrap();
            if func.is_inline() {
                lines.push(Line::InlineAsm(func.get_inline(vec![lhs.0, output.0])));
            } else {
                lines.push(Line::ReturnCall(
                    func.get_id(),
                    vec![(lhs.0, name_handler.type_table().get_type_size(lhs.1)?)],
                    output.0,
                ));
            }
            output
        }
        op_ => {
            let rhs = rhs.ok_or(ProcessorError::BadOperatorPosition(
                op.1.clone(),
                op.0.clone(),
            ))?;
            let func_name = match op_ {
                Operator::Add => "add",
                Operator::Subtract => "sub",
                Operator::Product => "mul",
                Operator::Divide => "div",
                Operator::Modulo => "mod",
                Operator::Greater => "gt",
                Operator::Less => "lt",
                Operator::GreaterEqual => "ge",
                Operator::LessEqual => "le",
                Operator::Equal => "eq",
                Operator::NotEqual => "ne",
                Operator::Or => "or",
                Operator::And => "and",
                Operator::Not => panic!(),
            };

            let func = function_holder.get_function(Some(lhs.1), func_name).ok_or(
                ProcessorError::OpFunctionNotFound(
                    op.1.clone(),
                    func_name.to_string(),
                    name_handler
                        .type_table
                        .get_type(lhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                    name_handler
                        .type_table
                        .get_type(rhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                ),
            )?;
            name_handler.use_function(func);
            let func_args = func.get_args();
            let func_id = func.get_id();

            if func_args.len() != 2 {
                return Err(ProcessorError::OpFunctionNotFound(
                    op.1.clone(),
                    func_name.to_string(),
                    name_handler
                        .type_table
                        .get_type(lhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                    name_handler
                        .type_table
                        .get_type(rhs.1)
                        .unwrap()
                        .get_name()
                        .to_string(),
                ));
            }

            let output = if let Some(return_into) = return_into {
                return_into
            } else {
                instantiate_literal::instantiate_literal(
                    Right(
                        func.get_return_type()
                            .ok_or(ProcessorError::OpFunctionNotFound(
                                op.1.clone(),
                                func_name.to_string(),
                                name_handler
                                    .type_table
                                    .get_type(lhs.1)
                                    .unwrap()
                                    .get_name()
                                    .to_string(),
                                name_handler
                                    .type_table
                                    .get_type(rhs.1)
                                    .unwrap()
                                    .get_name()
                                    .to_string(),
                            ))?,
                    ),
                    lines,
                    name_handler,
                    function_holder,
                    None,
                )?
            };

            let func = function_holder.functions().get(&func_id).unwrap();
            if func.is_inline() {
                lines.push(Line::InlineAsm(
                    func.get_inline(vec![lhs.0, rhs.0, output.0]),
                ));
            } else {
                lines.push(Line::ReturnCall(
                    func.get_id(),
                    vec![
                        (lhs.0, name_handler.type_table().get_type_size(lhs.1)?),
                        (rhs.0, name_handler.type_table().get_type_size(rhs.1)?),
                    ],
                    output.0,
                ));
            }
            output
        }
    }))
}
