use crate::parser::parse::parse;
use crate::processor::processor::process;
use std::path::PathBuf;
use crate::assembler::assemble::{assemble, generate_assembly};

mod ast;
mod basic_ast;
mod parser;
mod processor;
mod assembler;
mod compiler;

// TODO: Handle circular imports

fn main() {
    let mut asts = Vec::new();
    if let Err(e) = parse(PathBuf::from("main.why"), &mut asts) {
        println!("Parse Error:\n{}", e);
        return;
    } else {
        println!("Parse Result:\n{:?}", asts);
    }

    let functions = match process(asts) {
        Err(e) => {
            println!("Processing Error:\n{}", e);
            return
        }
        Ok(functions) => functions
    };

    generate_assembly(&PathBuf::from("output"), functions);
    #[cfg(target_os = "windows")]
    assemble(&PathBuf::from("output"));
    #[cfg(not(target_os = "windows"))]
    println!("Assembly, linking and execution omitted due to unsupported platform")
}
