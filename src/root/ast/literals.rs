use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::parser::line_info::LineInfo;

#[derive(Clone, strum_macros::Display, Debug)]
pub enum Literal {
    String(String),
    Char(char),
    Int(i128),
    Bool(bool),
    Initialiser(String, Vec<Vec<(BasicSymbol, LineInfo)>>),
    Null,
    None,
}

impl Literal {
    pub fn get_type_id(&self) -> (isize, usize) {
        match &self {
            Literal::Int(_) => (-1, 0),
            Literal::Bool(_) => (-2, 0),
            Literal::Null => (-1, 1),
            _ => todo!(),
        }
    }
}