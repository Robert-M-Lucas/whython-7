use crate::root::nom_parser::parse::{Location, ParseResult, Span};

enum LineTokens<'a> {
    Initialisation(InitialisationToken<'a>),
    Assignment(AssignmentToken<'a>),
    If(IfToken<'a>),
    While(WhileToken<'a>),
    Return(&'a str),
    Break,
    NoOp(EvaluableToken<'a>)
}

pub struct EvaluableToken {
    location: Location,
    tokens: Vec<EvaluableTokens>
}

struct InitialisationToken<'a> {
    location: Location,
    name: &'a str,
    type_name: &'a str,
    value: EvaluableToken<'a>
}

struct AssignmentOperatorToken {
    location: Location,
    assignment_operator: AssignmentOperatorTokens
}

enum AssignmentOperatorTokens {
    None,
    Combination(OperatorTokens),
}

struct AssignmentToken<'a> {
    location: Location,
    name: &'a str,
    assignment_operator: AssignmentOperatorToken,
    value: EvaluableToken<'a>
}

struct IfToken<'a> {
    location: Location,
    if_condition: EvaluableToken<'a>,
    if_contents: Vec<LineTokens<'a>>,
    elif_condition_contents: Vec<(EvaluableToken<'a>, Vec<LineTokens<'a>>)>,
    else_contents: Option<Vec<LineTokens<'a>>>
}

struct WhileToken<'a> {
    location: Location,
    condition: EvaluableToken<'a>,
    contents: Vec<LineTokens<'a>>
}

enum EvaluableTokens<'a> {
    Name(NameToken<'a>),
    Literal(LiteralTokens<'a>),
    InfixOperator(EvaluableToken<'a>, OperatorToken, EvaluableToken<'a>),
    PrefixOperator(OperatorToken, EvaluableToken<'a>)
}

struct OperatorToken {
    location: Location,
    operator: OperatorTokens
}

enum OperatorTokens {
    Add,
    Subtract,
}

enum LiteralTokens<'a> {
    Bool(bool),
    String(&'a str)
}

pub fn evaluable(s: Span) -> ParseResult<(), EvaluableToken> {
    todo!()
}