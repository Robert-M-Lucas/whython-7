use std::path::PathBuf;
use crate::parser::parse::parse;
use crate::processor::processor::process;

mod parser;
mod basic_ast;
mod il;
mod ast;
mod processor;

fn main() {
    let mut asts = Vec::new();
    if let Err(e) = parse(PathBuf::from("main.why"), &mut asts) {
        println!("{}", e.to_string());
    }
    else {
        println!("{:?}", asts);
    }

    process(asts);
}
