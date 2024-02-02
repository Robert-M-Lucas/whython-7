use crate::ast::keywords::Keyword;
use crate::ast::literals::Literal;
use crate::ast::operators::Operator;
use crate::basic_ast::punctuation::Punctuation;
use std::path::PathBuf;
use thiserror::__private::AsDisplay;

pub type BasicAbstractSyntaxTree = (PathBuf, Vec<(BasicSymbol, usize)>);

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
    Name(Vec<String>),
    Keyword(Keyword),
}

impl BasicSymbol {
    pub fn instead_found(&self) -> String {
        match &self {
            BasicSymbol::AbstractSyntaxTree(_) => panic!(),
            BasicSymbol::Literal(literal) => "Literal".to_string(),
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
