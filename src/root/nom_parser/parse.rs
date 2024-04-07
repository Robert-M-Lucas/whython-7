use std::fs;
use std::fs::File;
use std::mem::take;
use std::path::PathBuf;
use std::rc::Rc;
use nom::bytes::complete::{is_not, take_till, take_until, take_while};
use nom::{FindSubstring, InputLength, InputTake, InputTakeAtPosition, IResult, Offset, Parser};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{recognize, value};
use nom::error::{convert_error, Error, ErrorKind, ParseError, VerboseError};
use nom::sequence::{pair, Tuple};
use nom_locate::LocatedSpan;
use nom_supreme::error::{ErrorTree, GenericErrorTree};
use nom_supreme::final_parser::final_parser;
use nom_supreme::tag::complete::tag;
use crate::root::compiler::compile_functions::Function;

type Span<'a> = LocatedSpan<&'a str, &'a Rc<PathBuf>>;

type ParseResult<'a, I=Span<'a>, O=Span<'a>, E=ErrorTree<Span<'a>>> = IResult<I, O, E>;

struct Location {
    path: Rc<PathBuf>,
    offset: usize,
    line: u32
}

impl Location {
    pub fn from_span(span: Span) -> Location {
        Location {
            path: span.extra.clone(),
            offset: span.location_offset(),
            line: span.location_line()
        }
    }
}

#[derive(Debug)]
enum TopLevelTokens {
    // Struct(StructToken<'a>),
    // Impl(ImplToken<'a>),
    // Function(FunctionToken<'a>),
    Test
}

struct StructToken<'a> {
    location: Location,
    name: &'a str,
    attributes: Vec<(&'a str, &'a str)>
}


struct ImplToken<'a> {
    location: Location,
    name: &'a str,
    functions: Vec<FunctionToken<'a>>
}


struct FunctionToken<'a> {
    location: Location,
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
    location: Location,
    tokens: Vec<EvaluableTokens<'a>>
}

struct InitialisationToken<'a> {
    location: Location,
    name: &'a str,
    type_name: &'a str,
    value: EvaluableToken<'a>
}

struct AssignmentOperatorToken<'a> {
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
    assignment_operator: AssignmentOperatorToken<'a>,
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

enum NameConnectors {
    NonStatic,
    Static
}

struct NameToken<'a> {
    location: Location,
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


fn peol_comment(s: Span) -> ParseResult
{
    pair(tag("//"), is_not("\n\r")).parse(s)
        .map(|(s, (_, y)): (Span, (Span, Span))| (s, y))
}

fn pinline_comment(s: Span) -> ParseResult {
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

fn parse_toplevel(s: Span) -> ParseResult<Span, Vec<TopLevelTokens>> {
    let mut s = s;
    let mut tokens = Vec::new();

    loop {
        let ns = s;
        let ns = discard_ignored(ns);

        if ns.is_empty() {
            return Ok((ns, tokens))
        }

        let (ns, token) = alt((
            parse_function,
            parse_function,
        ))
            .parse(ns)?;

        tokens.push(token);

        s = ns;
    }
}

pub fn take_till_whitespace<F, Input, Error: ParseError<Input>>(
    cond: F,
) -> impl Fn(Input) -> IResult<Input, Input, Error>
    where
        Input: InputTakeAtPosition<Item=char>,
        F: Fn(<Input as InputTakeAtPosition>::Item) -> bool,
{
    take_till(|c: char| c.is_whitespace())
}

// fn parse_struct(s: Span) -> ParseResult<Span, TopLevelTokens> {
//     let (s, _) = tag("struct").parse(s)?;
//     Err()
//
//     ()
// }

fn parse_function(s: Span) -> ParseResult<Span, TopLevelTokens> {
    println!("{:?}", s);
    tag("fn").parse(s).map(|(s, _)| (s, TopLevelTokens::Test))
}

pub fn parse(path: PathBuf) -> Result<(), ()> {
    let text = fs::read_to_string(&path).unwrap();
    let path = Rc::new(path);
    let base = Span::new_extra(&text, &path);


    println!("{:?}",
       parse_toplevel(base)
    );

    // parse_toplevel(Span::new_extra(&text, &path)).unwrap();

    Ok(())
}