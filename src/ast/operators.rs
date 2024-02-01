use crate::basic_ast::symbol::BasicSymbol;

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum Operator {
    Add,
    Subtract,
    Product,
    Divide,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
    Or,
    And,
    Not,
}

pub const ALL_SYMBOLS: [char; 10] = ['+', '-', '*', '/', '>', '<', '=', '|', '&', '!'];

impl Operator {
    pub fn get_operator(string: &str) -> Option<Operator> {
        match string {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Subtract),
            "*" => Some(Operator::Product),
            "/" => Some(Operator::Divide),
            ">" => Some(Operator::Greater),
            "<" => Some(Operator::Less),
            ">=" => Some(Operator::GreaterEqual),
            "<=" => Some(Operator::LessEqual),
            "==" => Some(Operator::Equal),
            "!=" => Some(Operator::NotEqual),
            "|" => Some(Operator::Or),
            "&" => Some(Operator::And),
            "!" => Some(Operator::Not),
            _ => None,
        }
    }
}