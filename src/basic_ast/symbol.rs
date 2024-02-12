use crate::ast::keywords::Keyword;
use crate::ast::literals::Literal;
use crate::ast::operators::Operator;
use crate::basic_ast::punctuation::Punctuation;
use crate::parser::line_info::LineInfo;


pub type BasicAbstractSyntaxTree = Vec<(BasicSymbol, LineInfo)>;

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum NameAccessType {
    Base,
    Static,
    Normal,
}

#[derive(Clone, strum_macros::Display, Debug)]
pub enum NameType {
    Normal,
    Function(Vec<Vec<(BasicSymbol, LineInfo)>>),
}

#[derive(Clone, strum_macros::Display, Debug)]
pub enum BasicSymbol {
    AbstractSyntaxTree(Vec<(BasicSymbol, LineInfo)>),
    Literal(Literal),
    Operator(Operator),
    Assigner(Option<Operator>),
    BracedSection(Vec<(BasicSymbol, LineInfo)>),
    BracketedSection(Vec<(BasicSymbol, LineInfo)>),
    SquareBracketedSection(Vec<(BasicSymbol, LineInfo)>),
    Punctuation(Punctuation),
    Keyword(Keyword),
    Name(Vec<(String, NameAccessType, NameType)>),
}

impl BasicSymbol {
    pub fn get_name_contents(&self) -> &Vec<(String, NameAccessType, NameType)> {
        match self {
            BasicSymbol::Name(inside) => inside,
            _ => panic!(),
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
