use crate::root::ast::literals::Literal;
use crate::root::ast::operators::Operator;
use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::compile_functions::instantiate_literal::instantiate_variable;
use crate::root::compiler::compile_functions::name_handler::NameHandler;
use crate::root::compiler::compile_functions::{FunctionHolder, Line};
use crate::root::parser::line_info::LineInfo;
use crate::root::processor::custom_types::{Bool, Int};
use crate::root::processor::processor::ProcessorError;
use crate::root::processor::type_builder::{Type, TypedFunction};
use either::{Left, Right};

pub fn evaluate_operator(symbol: &(BasicSymbol, LineInfo)) -> Result<&Operator, ProcessorError> {
    match &symbol.0 {
        BasicSymbol::Operator(operator) => Ok(operator),
        _ => Err(ProcessorError::BadEvaluableLayout(symbol.1.clone())),
    }
}

pub fn evaluate_operation(
    lhs: (isize, (isize, usize)),
    op: (&Operator, &LineInfo),
    rhs: Option<(isize, (isize, usize))>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, (isize, usize))>,
) -> Result<Option<(isize, (isize, usize))>, ProcessorError> {
    Ok(Some(match &op.0 {
        Operator::Not => {
            let func = function_holder.get_function(Some(lhs.1), "not").ok_or(
                ProcessorError::SingleOpFunctionNotFound(
                    op.1.clone(),
                    "not".to_string(),
                    name_handler
                        .type_table()
                        .get_type(lhs.1 .0)
                        .unwrap()
                        .get_indirect_name(lhs.1 .1)
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
                        .type_table()
                        .get_type(lhs.1 .0)
                        .unwrap()
                        .get_indirect_name(lhs.1 .1)
                        .to_string(),
                ));
            }
            let output = if let Some(return_into) = return_into {
                return_into
            } else {
                instantiate_variable(
                    Right(
                        func.get_return_type()
                            .ok_or(ProcessorError::SingleOpFunctionNotFound(
                                op.1.clone(),
                                "not".to_string(),
                                name_handler
                                    .type_table()
                                    .get_type(lhs.1 .0)
                                    .unwrap()
                                    .get_indirect_name(lhs.1 .1)
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
                    -(name_handler.local_variable_space() as isize),
                    vec![(lhs.0, name_handler.type_table().get_type_size(lhs.1)?)],
                    name_handler.type_table().get_type_size(output.1)?,
                    output.0,
                ));
            }
            output
        }
        op_ => {
            if matches!(op_, Operator::And) && rhs.is_none() {
                let return_into = if let Some(return_into) = return_into {
                    if return_into.1 .0 != lhs.1 .0 || return_into.1 .1 != lhs.1 .1 + 1 {
                        return Err(ProcessorError::BadEvaluatedType(
                            op.1.clone(),
                            name_handler
                                .type_table()
                                .get_type(return_into.1 .0)
                                .unwrap()
                                .get_indirect_name(return_into.1 .1)
                                .to_string(),
                            name_handler
                                .type_table()
                                .get_type(lhs.1 .0)
                                .unwrap()
                                .get_indirect_name(lhs.1 .1 + 1)
                                .to_string(),
                        ));
                    }
                    return_into
                } else {
                    (
                        name_handler.add_local_variable(None, (lhs.1 .0, lhs.1 .1 + 1), lines)?,
                        (lhs.1 .0, lhs.1 .1 + 1),
                    )
                };
                lines.push(Line::InlineAsm(Int::instantiate_local_ref(
                    lhs.0,
                    return_into.0,
                )));
                return Ok(Some(return_into));
            }

            // Get ref
            if matches!(op_, Operator::Product) && rhs.is_none() {
                let return_into = if let Some(return_into) = return_into {
                    if lhs.1 .1 == 0 {
                        return Err(ProcessorError::CantDerefNonRef(op.1.clone()));
                    }
                    if return_into.1 .0 != lhs.1 .0 || return_into.1 .1 != lhs.1 .1 - 1 {
                        return Err(ProcessorError::BadEvaluatedType(
                            op.1.clone(),
                            name_handler
                                .type_table()
                                .get_type(return_into.1 .0)
                                .unwrap()
                                .get_indirect_name(return_into.1 .1)
                                .to_string(),
                            name_handler
                                .type_table()
                                .get_type(lhs.1 .0)
                                .unwrap()
                                .get_indirect_name(lhs.1 .1 - 1)
                                .to_string(),
                        ));
                    }
                    return_into
                } else {
                    (
                        name_handler.add_local_variable(None, (lhs.1 .0, lhs.1 .1 + 1), lines)?,
                        (lhs.1 .0, lhs.1 .1 - 1),
                    )
                };

                lines.push(Line::DynFromCopy(
                    lhs.0,
                    return_into.0,
                    name_handler.type_table().get_type_size(return_into.1)?,
                ));
                return Ok(Some(return_into));
            }

            // Heap alloc
            if matches!(op_, Operator::HeapAlloc) && rhs.is_none() {
                let return_into = if let Some(return_into) = return_into {
                    if return_into.1 .0 != lhs.1 .0 || return_into.1 .1 != lhs.1 .1 + 1 {
                        return Err(ProcessorError::BadEvaluatedType(
                            op.1.clone(),
                            name_handler
                                .type_table()
                                .get_type(return_into.1 .0)
                                .unwrap()
                                .get_indirect_name(return_into.1 .1)
                                .to_string(),
                            name_handler
                                .type_table()
                                .get_type(lhs.1 .0)
                                .unwrap()
                                .get_indirect_name(lhs.1 .1 + 1)
                                .to_string(),
                        ));
                    }
                    return_into
                } else {
                    (
                        name_handler.add_local_variable(None, (lhs.1 .0, lhs.1 .1 + 1), lines)?,
                        (lhs.1 .0, lhs.1 .1 + 1),
                    )
                };
                let size = name_handler.type_table().get_type_size(lhs.1)?;
                lines.push(Line::HeapAlloc(size, return_into.0));
                lines.push(Line::DynToCopy(lhs.0, return_into.0, size));
                return Ok(Some(return_into));
            }

            // Heap dealloc
            if matches!(op_, Operator::HeapDealloc) && rhs.is_none() {
                let return_into = if let Some(return_into) = return_into {
                    if return_into.1 != (Bool::get_id(), 0) {
                        return Err(ProcessorError::BadEvaluatedType(
                            op.1.clone(),
                            name_handler
                                .type_table()
                                .get_type(return_into.1 .0)
                                .unwrap()
                                .get_indirect_name(return_into.1 .1)
                                .to_string(),
                            name_handler
                                .type_table()
                                .get_type(Bool::get_id())
                                .unwrap()
                                .get_indirect_name(0)
                                .to_string(),
                        ));
                    }
                    return_into
                } else {
                    (
                        name_handler.add_local_variable(None, (Bool::get_id(), 0), lines)?,
                        (Bool::get_id(), 0),
                    )
                };

                if lhs.1 .1 == 0 {
                    return Err(ProcessorError::CantDeallocateNonRef(op.1.clone()));
                }

                lines.push(Line::HeapDealloc(lhs.0, return_into.0));
                return Ok(Some(return_into));
            }

            let (lhs, rhs) = if matches!(op_, Operator::Subtract)
                && rhs.is_none()
                && lhs.1 == (Int::get_id(), 0)
            {
                (
                    instantiate_variable(
                        Left((&Literal::Int(0), &op.1)),
                        lines,
                        name_handler,
                        function_holder,
                        None,
                    )
                    .unwrap(),
                    lhs,
                )
            } else {
                (
                    lhs,
                    rhs.ok_or(ProcessorError::BadOperatorPosition(
                        op.1.clone(),
                        op.0.clone(),
                    ))?,
                )
            };

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
                Operator::HeapAlloc => "heap allocate",
                Operator::HeapDealloc => "heap deallocate",
                Operator::Not => panic!(),
            };

            let func = function_holder.get_function(Some(lhs.1), func_name).ok_or(
                ProcessorError::OpFunctionNotFound(
                    op.1.clone(),
                    func_name.to_string(),
                    name_handler
                        .type_table()
                        .get_type(lhs.1 .0)
                        .unwrap()
                        .get_indirect_name(lhs.1 .1)
                        .to_string(),
                    name_handler
                        .type_table()
                        .get_type(rhs.1 .0)
                        .unwrap()
                        .get_indirect_name(rhs.1 .1)
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
                        .type_table()
                        .get_type(lhs.1 .0)
                        .unwrap()
                        .get_indirect_name(lhs.1 .1)
                        .to_string(),
                    name_handler
                        .type_table()
                        .get_type(rhs.1 .0)
                        .unwrap()
                        .get_indirect_name(rhs.1 .1)
                        .to_string(),
                ));
            }

            let ret_type = func
                .get_return_type()
                .ok_or(ProcessorError::OpFunctionNotFound(
                    op.1.clone(),
                    func_name.to_string(),
                    name_handler
                        .type_table()
                        .get_type(lhs.1 .0)
                        .unwrap()
                        .get_indirect_name(lhs.1 .1)
                        .to_string(),
                    name_handler
                        .type_table()
                        .get_type(rhs.1 .0)
                        .unwrap()
                        .get_indirect_name(rhs.1 .1)
                        .to_string(),
                ))?;

            let output = if let Some(return_into) = return_into {
                if return_into.1 != ret_type {
                    return Err(ProcessorError::BadEvaluatedType(
                        op.1.clone(),
                        name_handler
                            .type_table()
                            .get_type(return_into.1 .0)
                            .unwrap()
                            .get_indirect_name(return_into.1 .1)
                            .to_string(),
                        name_handler
                            .type_table()
                            .get_type(ret_type.0)
                            .unwrap()
                            .get_indirect_name(ret_type.1)
                            .to_string(),
                    ));
                }
                return_into
            } else {
                instantiate_variable(Right(ret_type), lines, name_handler, function_holder, None)?
            };

            let func = function_holder.functions().get(&func_id).unwrap();
            if func.is_inline() {
                lines.push(Line::InlineAsm(
                    func.get_inline(vec![lhs.0, rhs.0, output.0]),
                ));
            } else {
                lines.push(Line::ReturnCall(
                    func.get_id(),
                    -(name_handler.local_variable_space() as isize),
                    vec![
                        (lhs.0, name_handler.type_table().get_type_size(lhs.1)?),
                        (rhs.0, name_handler.type_table().get_type_size(rhs.1)?),
                    ],
                    name_handler.type_table().get_type_size(output.1)?,
                    output.0,
                ));
            }
            output
        }
    }))
}
