use crate::ast::keywords::Keyword;
use crate::ast::literals::Literal;
use crate::ast::operators::Operator;
use crate::basic_ast::punctuation::Punctuation;
use std::path::PathBuf;


pub type BasicAbstractSyntaxTree = (PathBuf, Vec<(BasicSymbol, usize)>);

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum NameAccessType {
    Base,
    Static,
    Normal
}

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum NameType {
    Normal,
    Function(Vec<Vec<BasicSymbol>>)
}

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum BasicSymbol {
    AbstractSyntaxTree(Vec<(BasicSymbol, usize)>),
    Literal(Literal),
    Operator(Operator),
    Assigner(Operator),
    BracedSection(Vec<(BasicSymbol, usize)>),
    BracketedSection(Vec<(BasicSymbol, usize)>),
    SquareBracketedSection(Vec<(BasicSymbol, usize)>),
    Punctuation(Punctuation),
    Keyword(Keyword),
    Name(Vec<(String, NameAccessType, NameType)>),
}

impl BasicSymbol {
    pub fn get_name_contents(&self) -> &Vec<(String, NameAccessType, NameType)> {
        match self {
            BasicSymbol::Name(inside) => inside,
            _ => panic!()
        }
    }
}

impl BasicSymbol {
    pub fn instead_found(&self) -> String {
        match &self {
            BasicSymbol::AbstractSyntaxTree(_) => panic!(),
            BasicSymbol::Literal(_literal) => "Literal".to_string(),
            BasicSymbol::Operator(_) => "Operator".to_string(),
            BasicSymbol::Assigner(_) => "Assigner".to_string(),
            BasicSymbol::BracedSection(_) => "BracedSection".to_string(),
            BasicSymbol::BracketedSection(_) => "BracketedSection".to_string(),
            BasicSymbol::SquareBracketedSection(_) => "SquareBracketedSection".to_string(),
            BasicSymbol::Punctuation(punctuation) => {
                format!("{punctuation}")
            }
            BasicSymbol::Name(_) => "Name".to_string(),
            BasicSymbol::Keyword(_) => "Keyword".to_string(),
        }
    }
}
