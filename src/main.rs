use std::path::PathBuf;
use crate::parser::parse::parse;

mod parser;
mod basic_ast;
mod il;
mod ast;

fn main() {
    let mut asts = Vec::new();
    if let Err(e) = parse(PathBuf::from("main.why"), &mut asts) {
        println!("{}", e.to_string());
    }
    else {
        println!("{:?}", asts);
    }
}
