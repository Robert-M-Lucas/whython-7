use std::fs;
use std::mem::take;
use std::panic::Location;
use std::path::PathBuf;
use nom::bytes::complete::{is_not, tag, take_until, take_while};
use nom::{FindSubstring, InputLength, InputTake, IResult, Parser};
use nom::character::complete::char;
use nom::combinator::{recognize, value};
use nom::error::{Error, ErrorKind, ParseError};
use nom::sequence::{pair, Tuple};
use nom_locate::LocatedSpan;
use crate::root::compiler::compile_functions::Function;

type Span<'a> = LocatedSpan<&'a str, &'a PathBuf>;
type ParseResult<'a, O=Span<'a>, E=nom::error::Error<Span<'a>>> = IResult<Span<'a>, O, E>;

enum TopLevelTokens<'a> {
    Struct(StructToken<'a>),
    Impl(ImplToken<'a>),
    Function(FunctionToken<'a>)
}

struct StructToken<'a> {
    location: Span<'a>,
    name: &'a str,
    attributes: Vec<(&'a str, &'a str)>
}


struct ImplToken<'a> {
    location: Span<'a>,
    name: &'a str,
    functions: Vec<FunctionToken<'a>>
}


struct FunctionToken<'a> {
    location: Span<'a>,
    name: &'a str,
    return_type: &'a str,
    arguments: Vec<(&'a str, &'a str)>,
    contents: Vec<LineTokens<'a>>
}

enum LineTokens<'a> {
    Initialisation(InitialisationToken<'a>),
    Assignment(AssignmentToken<'a>),
    If(IfToken<'a>),
    While(WhileToken<'a>),
    Return(&'a str),
    Break,
    NoOp(EvaluableToken<'a>)
}

struct EvaluableToken<'a> {
    location: Span<'a>,
    tokens: Vec<EvaluableTokens<'a>>
}

struct InitialisationToken<'a> {
    location: Span<'a>,
    name: &'a str,
    type_name: &'a str,
    value: EvaluableToken<'a>
}

struct AssignmentOperatorToken<'a> {
    location: Span<'a>,
    assignment_operator: AssignmentOperatorTokens
}

enum AssignmentOperatorTokens {
    None,
    Combination(OperatorTokens),
}

struct AssignmentToken<'a> {
    location: Span<'a>,
    name: &'a str,
    assignment_operator: AssignmentOperatorToken<'a>,
    value: EvaluableToken<'a>
}

struct IfToken<'a> {
    location: Span<'a>,
    if_condition: EvaluableToken<'a>,
    if_contents: Vec<LineTokens<'a>>,
    elif_condition_contents: Vec<(EvaluableToken<'a>, Vec<LineTokens<'a>>)>,
    else_contents: Option<Vec<LineTokens<'a>>>
}

struct WhileToken<'a> {
    location: Span<'a>,
    condition: EvaluableToken<'a>,
    contents: Vec<LineTokens<'a>>
}

enum NameConnectors {
    NonStatic,
    Static
}

struct NameToken<'a> {
    location: Span<'a>,
    base: &'a str,
    names: Vec<(NameConnectors, &'a str)>,
    function_call: Option<Vec<EvaluableToken<'a>>>
}

enum EvaluableTokens<'a> {
    Name(NameToken<'a>),
    Literal(LiteralTokens<'a>),
    InfixOperator(EvaluableToken<'a>, OperatorToken<'a>, EvaluableToken<'a>),
    PrefixOperator(OperatorToken<'a>, EvaluableToken<'a>)
}

struct OperatorToken<'a> {
    location: Span<'a>,
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

// fn parse_struct(s: &str) -> ParseResult {
//
// }
//
// fn parse_function(s: &str) -> ParseResult {
//
// }

fn peol_comment(s: Span) -> ParseResult
{
    pair(tag::<&str, LocatedSpan<&str, &PathBuf>, nom::error::Error<Span>>("//"), is_not("\n\r")).parse(s)
        .map(|(s, (_, y)): (Span, (Span, Span))| (s, y))
}

fn pinline_comment(s: Span) -> ParseResult{
    (
        tag("/*"),
        take_until("*/"),
        tag("*/")
    ).parse(s)
        .map(|(s, (_, y, _)): (Span, (Span, Span, Span))| (s, y))
}

pub fn parse_comment(s: Span) -> ParseResult {
    pinline_comment(s).or_else(|_| peol_comment(s))
}

pub fn take_whitespace(s: Span) -> ParseResult {
    take_while(|c: char| c.is_whitespace())(s)
}

pub fn discard_ignored(s: Span) -> Span {
    let mut s = s;
    let mut found = true;
    while found {
        found = false;
        if let Ok((ns, _)) = parse_comment(s) {
            s = ns;
            found = true;
        }
        if let Ok((ns, p)) = take_whitespace(s) {
            if !p.is_empty() {
                s = ns;
                found = true;
            }
        }
    }

    s
}

//
// fn parse_toplevel(s: Span) -> ParseResult<Vec<TopLevelTokens>> {
//     loop {
//         let (s, parsed) = take_until()
//     }
// }

pub fn parse(path: PathBuf) -> Result<(), ()> {
    // let text = fs::read_to_string(&path).unwrap();
    let text = String::from("asd // test_string");

    println!("{:?}",
        discard_ignored(Span::new_extra(&text, &path))
    );

    // parse_toplevel(Span::new_extra(&text, &path)).unwrap();

    Ok(())
}