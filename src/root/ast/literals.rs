use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::custom::types::bool::Bool;
use crate::root::custom::types::float::Float;
use crate::root::custom::types::int::Int;
use crate::root::parser::line_info::LineInfo;
use crate::root::processor::processor::ProcessorError;
use crate::root::processor::type_builder::{Type, TypeTable};

#[derive(Clone, strum_macros::Display, Debug)]
#[allow(dead_code)]
pub enum Literal {
    String(String),
    Char(char),
    Int(i128),
    Bool(bool),
    Float(f64),
    Initialiser(String, Vec<Vec<(BasicSymbol, LineInfo)>>),
    Null,
    None,
}

impl Literal {
    pub fn get_type_id(&self, type_table: &TypeTable, line_info: &LineInfo) -> Result<(isize, usize), ProcessorError> {
        Ok(match &self {
            Literal::Int(_) => (Int::get_id(), 0),
            Literal::Bool(_) => (Bool::get_id(), 0),
            Literal::Float(_) => (Float::get_id(), 0),
            Literal::Null => (-1, 1),
            Literal::Initialiser(name, _) => (
                type_table.get_id_by_name(name).ok_or_else(||
                    ProcessorError::TypeNotFound(line_info.clone(), name.clone())
                )?, 
                0),
            _ => todo!(),
        })
    }
}
