use crate::root::nom_parser::parse::{FunctionToken, Location};

struct ImplToken<'a> {
    location: Location,
    name: &'a str,
    functions: Vec<FunctionToken<'a>>
}
