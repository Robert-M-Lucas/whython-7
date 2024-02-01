use std::path::PathBuf;
use crate::ast::keywords::Keyword;
use crate::ast::literals::Literal;
use crate::ast::operators::Operator;
use crate::basic_ast::punctuation::Punctuation;

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum BasicSymbol {
    AbstractSyntaxTree(Vec<BasicSymbol>),
    Literal(Literal),
    Operator(Operator),
    Assigner(Operator),
    BracedSection(Vec<BasicSymbol>),
    BracketedSection(Vec<BasicSymbol>),
    SquareBracketedSection(Vec<BasicSymbol>),
    Punctuation(Punctuation),
    Name(Vec<String>),
    Keyword(Keyword),
    Line(Vec<BasicSymbol>),
}