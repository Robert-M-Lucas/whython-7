use crate::parser::parse::parse;
use crate::processor::processor::process;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::time::Instant;
use crate::assembler::assemble::{assemble, generate_assembly, link};
use crate::parser::line_info::LineInfo;

mod ast;
mod basic_ast;
mod parser;
mod processor;
mod assembler;
mod compiler;

// TODO: Handle circular imports

macro_rules! time {
    ($($tts:tt)*) => {
        let t = Instant::now();
        $($tts)*;
        let end = t.elapsed();
        println!("Completed [{:?}]", end);
    };
}

fn main() {
    print!("{}", LineInfo{
        file: Rc::new(String::from("main.why")),
        line: 2,
        char_start: 22
    });
    return;

    let mut asts = Vec::new();
    print!("Parsing...");
    time!(
        if let Err(e) = parse(PathBuf::from("main.why"), &mut asts) {
            println!("Parse Error:\n{}", e);
            return;
        } else {
            println!("Parse Result:\n{:?}", asts);
        }
    );

    print!("Processing...");
    time!(
        let functions = match process(asts) {
            Err(e) => {
                println!("Processing Error:\n{}", e);
                return
            }
            Ok(functions) => functions
        }
    );
    // let functions = process(asts).unwrap();

    print!("Compiling...");
    time!(generate_assembly(&PathBuf::from("output"), functions));
    #[cfg(target_os = "windows")]
    print!("Assembling (NASM)...");
    time!(assemble());
    println!("Linking (MSVC)...");
    time!(link());
    println!("Executing...");
    time!(run());
    #[cfg(not(target_os = "windows"))]
    println!("Assembling, linking, and execution omitted due to unsupported platform")
}

fn run() {
    println!("\nExited with return code {}",
             Command::new(".\\output\\out.exe")
                 .status()
                 .unwrap().code().unwrap())
}