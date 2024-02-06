#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum Literal {
    String(String),
    Char(char),
    Int(i128),
    Bool(bool),
    None,
}

impl Literal {
    pub fn get_type_id(&self) -> isize {
        todo!()
    }
}