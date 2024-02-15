use crate::basic_ast::symbol::BasicSymbol;
use crate::compiler::compile_functions::{evaluate, FunctionHolder, Line, NameHandler};
use crate::parser::line_info::LineInfo;
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::{Type, TypedFunction};

pub fn call_function(
    function: &Box<dyn TypedFunction>,
    start_line: &LineInfo,
    default_arg: Option<(isize, isize)>,
    args: &Vec<Vec<(BasicSymbol, LineInfo)>>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<Option<(isize, isize)>, ProcessorError> {
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
        if default_arg.1 != target_args[0].1 {
            return Err(ProcessorError::Placeholder2); // TODO:
        }
        call_args.push((
            default_arg.0,
            name_handler
                .type_table()
                .get_type_size(default_arg.1)
                .unwrap(),
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
                    .get_type(target_args[call_args.len()].1)
                    .unwrap()
                    .get_name()
                    .to_string(),
                name_handler
                    .type_table()
                    .get_type(evaluated.1)
                    .unwrap()
                    .get_name()
                    .to_string(),
                function.get_line(),
            ));
        }
        call_args.push((
            evaluated.0,
            name_handler
                .type_table()
                .get_type_size(evaluated.1)
                .unwrap(),
        ));
    }

    Ok(if let Some(return_type) = function.get_return_type() {
        if return_into.is_some() && return_into.unwrap().1 != return_type {
            return Err(ProcessorError::BadEvaluatedType(
                start_line.clone(),
                name_handler
                    .type_table()
                    .get_type(return_into.unwrap().1)
                    .unwrap()
                    .get_name()
                    .to_string(),
                name_handler
                    .type_table()
                    .get_type(return_type)
                    .unwrap()
                    .get_name()
                    .to_string(),
            ));
        }
        let return_into = if let Some(return_into) = return_into {
            (
                return_into.0,
                name_handler
                    .type_table()
                    .get_type_size(return_type)
                    .unwrap(),
            )
        } else {
            (
                name_handler.add_local_variable(None, return_type)?,
                name_handler.type_table.get_type_size(return_type)?,
            )
        };

        if function.is_inline() {
            let mut inline_args: Vec<_> = call_args.into_iter().map(|x| x.0).collect();
            inline_args.push(return_into.0);
            lines.push(Line::InlineAsm(function.get_inline(inline_args)));
        } else {
            lines.push(Line::ReturnCall(
                function.get_id(),
                call_args,
                return_into.0,
            ))
        }

        Some((return_into.0, return_type))
    } else {
        if return_into.is_some() {
            return Err(ProcessorError::BadEvaluatedType(
                start_line.clone(),
                name_handler
                    .type_table()
                    .get_type(return_into.unwrap().1)
                    .unwrap()
                    .get_name()
                    .to_string(),
                "None".to_string(),
            ));
        }

        if function.is_inline() {
            let inline_args: Vec<_> = call_args.into_iter().map(|x| x.0).collect();
            lines.push(Line::InlineAsm(function.get_inline(inline_args)));
        } else {
            lines.push(Line::NoReturnCall(function.get_id(), call_args))
        }

        None
    })
}
