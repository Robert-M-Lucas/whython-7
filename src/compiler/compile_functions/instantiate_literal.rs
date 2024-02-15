use either::{Either, Left, Right};
use crate::ast::literals::Literal;
use crate::compiler::compile_functions::{FunctionHolder, Line, NameHandler};
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::Type;

fn try_instantiate_literal(
    literal: Either<(isize, isize), &Literal>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<(isize, isize), ProcessorError> {
    match literal {
        Left(r) => Ok(r),
        Right(literal) => instantiate_literal(
            Left(literal),
            lines,
            name_handler,
            function_holder,
            return_into,
        ),
    }
}

pub fn instantiate_literal(
    literal: Either<&Literal, isize>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    _function_holder: &FunctionHolder,
    return_into: Option<(isize, isize)>,
) -> Result<(isize, isize), ProcessorError> {
    let (addr, id) = if let Some((addr, id)) = return_into {
        (addr, id)
    } else {
        let id = match &literal {
            Left(literal) => literal.get_type_id(),
            Right(id) => *id,
        };
        (name_handler.add_local_variable(None, id)?, id)
    };
    let _type = name_handler.type_table().get_type(id).unwrap();
    let asm = match literal {
        Left(literal) => _type.instantiate(Some(literal), addr)?,
        Right(_id) => _type.instantiate(None, addr)?,
    };
    lines.push(Line::InlineAsm(asm));
    Ok((addr, id))
}
