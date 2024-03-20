use crate::root::ast::operators::Operator;
use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::compile_functions::name_handler::NameHandler;
use crate::root::compiler::compile_functions::operators::evaluate_operation;
use crate::root::compiler::compile_functions::{evaluate, FunctionHolder, Line};
use crate::root::parser::line_info::LineInfo;
use crate::root::processor::processor::ProcessorError;
use crate::root::processor::type_builder::TypedFunction;

pub fn call_function(
    function: &Box<dyn TypedFunction>,
    start_line: &LineInfo,
    default_arg: Option<(isize, (isize, usize))>,
    args: &Vec<Vec<(BasicSymbol, LineInfo)>>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, (isize, usize))>,
) -> Result<Option<(isize, (isize, usize))>, ProcessorError> {
    name_handler.use_function(function);
    let target_args = function.get_args();
    let mut args_len = args.len();
    if default_arg.is_some() {
        args_len += 1;
    }

    if args_len > target_args.len() {
        return Err(ProcessorError::BadArgCount(
            // TODO: Bad line location
            args[target_args.len() - (args_len - args.len())][0]
                .1
                .clone(),
            target_args.len(),
            args_len,
            function.get_line(),
        ));
    }
    if args_len < target_args.len() {
        if args.is_empty() {
            return Err(ProcessorError::BadArgCount(
                // TODO: Bad line location
                start_line.clone(),
                target_args.len(),
                args_len,
                function.get_line(),
            ));
        } else {
            return Err(ProcessorError::BadArgCount(
                // TODO: Bad line location
                args[args.len() - 1].last().unwrap().1.clone(),
                target_args.len(),
                args_len,
                function.get_line(),
            ));
        }
    }

    let mut call_args = Vec::new();
    if let Some(default_arg) = default_arg {
        let default_arg = if default_arg.1 .1 == 0 && target_args[0].1 .1 == 1 {
            // TODO: Bad operator line
            let into = name_handler.add_local_variable(None, target_args[0].1, lines)?;
            evaluate_operation(
                default_arg,
                (&Operator::And, start_line),
                None,
                lines,
                name_handler,
                function_holder,
                Some((into, target_args[0].1)),
            )?
            .unwrap()
        } else {
            default_arg
        };
        if default_arg.1 != target_args[0].1 {
            panic!("Default arg doesn't match first target arg")
        }
        call_args.push((
            default_arg.0,
            name_handler.type_table().get_type_size(default_arg.1)?,
        ));
    }
    for arg in args {
        let evaluated = evaluate::evaluate(arg, lines, name_handler, function_holder, None)?;
        // println!("{:?}", evaluated);
        if evaluated.is_none() {
            return Err(ProcessorError::DoesntEvaluate(arg[0].1.clone()));
        }
        let evaluated = evaluated.unwrap();
        if evaluated.1 != target_args[call_args.len()].1 {
            return Err(ProcessorError::BadArgType(
                arg[0].1.clone(),
                name_handler
                    .type_table()
                    .get_type(target_args[call_args.len()].1 .0)
                    .unwrap()
                    .get_indirect_name(target_args[call_args.len()].1 .1)
                    .to_string(),
                name_handler
                    .type_table()
                    .get_type(evaluated.1 .0)
                    .unwrap()
                    .get_indirect_name(evaluated.1 .1)
                    .to_string(),
                function.get_line(),
            ));
        }
        call_args.push((
            evaluated.0,
            name_handler.type_table().get_type_size(evaluated.1)?,
        ));
    }

    Ok(if let Some(return_type) = function.get_return_type() {
        if return_into.is_some() && return_into.unwrap().1 != return_type {
            return Err(ProcessorError::BadEvaluatedType(
                start_line.clone(),
                name_handler
                    .type_table()
                    .get_type(return_into.unwrap().1 .0)
                    .unwrap()
                    .get_indirect_name(return_into.unwrap().1 .1)
                    .to_string(),
                name_handler
                    .type_table()
                    .get_type(return_type.0)
                    .unwrap()
                    .get_indirect_name(return_type.1)
                    .to_string(),
            ));
        }
        let return_into = if let Some(return_into) = return_into {
            (
                return_into.0,
                name_handler.type_table().get_type_size(return_type)?,
            )
        } else {
            (
                name_handler.add_local_variable(None, return_type, lines)?,
                name_handler.type_table().get_type_size(return_type)?,
            )
        };

        if function.is_inline() {
            let mut inline_args: Vec<_> = call_args.into_iter().map(|x| x.0).collect();
            inline_args.push(return_into.0);
            lines.push(Line::InlineAsm(function.get_inline(inline_args)));
        } else {
            lines.push(Line::ReturnCall(
                function.get_id(),
                -(name_handler.local_variable_space() as isize),
                call_args,
                name_handler.type_table().get_type_size(return_type)?,
                return_into.0,
            ))
        }

        Some((return_into.0, return_type))
    } else {
        if let Some(return_into) = return_into {
            return Err(ProcessorError::BadEvaluatedType(
                start_line.clone(),
                name_handler
                    .type_table()
                    .get_type(return_into.1 .0)
                    .unwrap()
                    .get_indirect_name(return_into.1 .1)
                    .to_string(),
                "None".to_string(),
            ));
        }

        if function.is_inline() {
            let inline_args: Vec<_> = call_args.into_iter().map(|x| x.0).collect();
            lines.push(Line::InlineAsm(function.get_inline(inline_args)));
        } else {
            lines.push(Line::NoReturnCall(function.get_id(), -(name_handler.local_variable_space() as isize), call_args, 0))
        }

        None
    })
}
