use std::path::PathBuf;
use crate::parser::parse::parse;
use crate::processor::processor::process;

mod parser;
mod basic_ast;
mod il;
mod ast;
mod processor;

// TODO: Handle circular imports

fn main() {
    let mut asts = Vec::new();
    if let Err(e) = parse(PathBuf::from("main.why"), &mut asts) {
        println!("Parse Error:\n{}", e.to_string());
    }
    else {
        println!("Parse Result:\n{:?}", asts);
    }

    if let Err(e) = process(asts) {
        println!("Processing Error:\n{}", e.to_string());
    }
    else {
    }
}
