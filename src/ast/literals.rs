use crate::basic_ast::symbol::BasicSymbol;
use crate::parser::line_info::LineInfo;

#[derive(Clone, strum_macros::Display, Debug)]
pub enum Literal {
    String(String),
    Char(char),
    Int(i128),
    Bool(bool),
    Initialiser(String, Vec<Vec<(BasicSymbol, LineInfo)>>),
    None,
}

impl Literal {
    pub fn get_type_id(&self) -> isize {
        match &self {
            Literal::Int(_) => -1,
            Literal::Bool(_) => -2,
            _ => todo!(),
        }
    }
}
