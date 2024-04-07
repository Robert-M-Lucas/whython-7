use nom::branch::alt;
use nom::Parser;
use crate::root::nom_parser::parse::{ParseResult, Span};
use crate::root::nom_parser::{parse_util};
use crate::root::nom_parser::parse_fn::parse_function;
use crate::root::nom_parser::parse_struct::parse_struct;

#[derive(Debug)]
pub enum TopLevelTokens {
    // Struct(StructToken<'a>),
    // Impl(ImplToken<'a>),
    // Function(FunctionToken<'a>),
    Test
}

pub fn parse_toplevel(s: Span) -> ParseResult<Span, Vec<TopLevelTokens>> {
    let mut s = s;
    let mut tokens = Vec::new();

    loop {
        let ns = s;
        let ns = parse_util::discard_ignored(ns);

        if ns.is_empty() {
            return Ok((ns, tokens))
        }

        let (ns, token) = alt((
            parse_function,
            parse_struct
        ))
            .parse(ns)?;

        tokens.push(token);

        s = ns;
    }
}
