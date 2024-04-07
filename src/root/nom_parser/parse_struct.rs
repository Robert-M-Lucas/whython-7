use nom_supreme::tag::complete::tag;
use nom::Parser;
use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_blocks::braced_section;
use crate::root::nom_parser::parse_name::parse_simple_name;
use crate::root::nom_parser::parse_toplevel::TopLevelTokens;
use crate::root::nom_parser::parse_util::require_ignored;

struct StructToken<'a> {
    location: Location,
    name: &'a str,
    attributes: Vec<(&'a str, &'a str)>
}

pub fn parse_struct(s: Span) -> ParseResult<Span, TopLevelTokens> {
    let (s, _) = tag("struct").parse(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, contents) = braced_section(s)?;

    todo!()
}
