use crate::parser::parse::parse;
use crate::processor::processor::process;
use std::path::PathBuf;

mod ast;
mod basic_ast;
mod parser;
mod processor;
mod assembler;

// TODO: Handle circular imports

fn main() {
    let mut asts = Vec::new();
    if let Err(e) = parse(PathBuf::from("main.why"), &mut asts) {
        println!("Parse Error:\n{}", e);
    } else {
        println!("Parse Result:\n{:?}", asts);
    }

    if let Err(e) = process(asts) {
        println!("Processing Error:\n{}", e);
    } 
}
