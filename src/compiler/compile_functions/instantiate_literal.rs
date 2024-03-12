use crate::ast::literals::Literal;
use crate::compiler::compile_functions::{FunctionHolder, Line};
use crate::processor::processor::ProcessorError;
use crate::processor::type_builder::Type;
use either::{Either, Left, Right};
use crate::compiler::compile_functions::evaluate::evaluate;
use crate::compiler::compile_functions::name_handler::NameHandler;
use crate::parser::line_info::LineInfo;
use crate::processor::custom_types::Int;

fn try_instantiate_literal(
    literal: Either<(isize, (isize, usize)), (&Literal, &LineInfo)>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, (isize, usize))>,
) -> Result<(isize, (isize, usize)), ProcessorError> {
    match literal {
        Left(r) => Ok(r),
        Right(literal) => instantiate_variable(
            Left(literal),
            lines,
            name_handler,
            function_holder,
            return_into,
        ),
    }
}

pub fn instantiate_variable(
    literal: Either<(&Literal, &LineInfo), (isize, usize)>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<(isize, (isize, usize))>,
) -> Result<(isize, (isize, usize)), ProcessorError> {
    let (addr, id) = if let Some((addr, id)) = return_into {
        (addr, id)
    } else {
        let id = match &literal {
            Left((literal, _)) => (literal.get_type_id(), 0),
            Right(id) => *id,
        };
        (name_handler.add_local_variable(None, id)?, id)
    };
    // Indirect
    let true_id = id.0;
    let indirection = id.1;
    let id = if indirection != 0 {
        Int::get_id()
    }
    else { 
        id.0
    };
    let _type = name_handler.type_table().get_type(id).unwrap();
    let asm = match &literal {
        Left((Literal::Initialiser(name, attributes), line_info)) => {
            let line_info = *line_info;
            let _id = name_handler.type_table().get_id_by_name(name)
                .ok_or(ProcessorError::TypeNotFound(line_info.clone(), name.clone()))?;

            if id != _id {
                return Err(
                    ProcessorError::BadEvaluatedType(
                        line_info.clone(),
                        _type.get_name().to_string(),
                        name.clone()
                    )
                );
            }

            if id < 0 {
                return Err(
                    ProcessorError::AttemptedBuiltinInitialiser(line_info.clone())
                )
            }

            // let size = _type.get_size(name_handler.type_table(), None)?; // ! Ensures non-circularity
            let attribute_types = _type.get_user_type().unwrap().get_attribute_types();
            if attributes.len() != attribute_types.len() {
                return Err(ProcessorError::IncorrectAttribCount(line_info.clone(), attribute_types.len(), attributes.len()));
            }

            let mut addr_counter = addr;
            for (attribute, attribute_type) in attributes.iter().zip(attribute_types) {
                evaluate(attribute, lines, name_handler, function_holder, Some((addr_counter, attribute_type)))?;

                addr_counter += name_handler.type_table().get_type_size(attribute_type)? as isize;
            }

            Vec::new()
        }
        Left((literal, _line_info)) => _type.instantiate(Some(literal), addr)?,
        Right(_id) => _type.instantiate(None, addr)?,
    };
    lines.push(Line::InlineAsm(asm));
    Ok((addr, (true_id, indirection)))
}
