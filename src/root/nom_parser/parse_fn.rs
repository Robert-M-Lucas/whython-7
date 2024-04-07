pub mod base;

use nom_supreme::tag::complete::tag;
use nom::Parser;
use crate::root::nom_parser::parse_toplevel::TopLevelTokens;

struct FunctionToken<'a> {
    location: Location,
    name: &'a str,
    return_type: &'a str,
    arguments: Vec<(&'a str, &'a str)>,
    contents: Vec<LineTokens<'a>>
}

pub fn parse_function(s: Span) -> ParseResult<Span, TopLevelTokens> {
    println!("{:?}", s);
    tag("fn").parse(s).map(|(s, _)| (s, TopLevelTokens::Test))
}
