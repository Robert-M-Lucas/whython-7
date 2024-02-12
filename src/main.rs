use crate::assembler::assemble::{assemble, generate_assembly, link};

use crate::parser::parse::parse;
use crate::processor::processor::process;
use std::path::PathBuf;
use std::process::Command;

use std::time::Instant;

mod assembler;
mod ast;
mod basic_ast;
mod compiler;
mod parser;
mod processor;
mod custom;

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
    let mut asts = Vec::new();
    print!("Parsing...");
    time!(
        if let Err(e) = parse(PathBuf::from("main.why"), &mut asts) {
            println!("\n{}", e);
            return;
        }
    );

    print!("Processing...");
    time!(
        let functions = match process(asts) {
            Err(e) => {
                println!("\n{}", e);
                return
            }
            Ok(functions) => functions
        }
    );
    // let functions = process(asts).unwrap();

    print!("Compiling...");
    time!(generate_assembly(&PathBuf::from("output"), functions));
    print!("Assembling (NASM)...");
    time!(assemble());
    #[cfg(target_os = "windows")]
    {
        println!("Linking (MSVC)...");
        time!(link());
        println!("Executing...");
        time!(run());
    }
    #[cfg(not(target_os = "windows"))]
    println!("Linking and execution omitted due to unsupported platform")
}

fn run() {
    println!(
        "\nExited with return code {}",
        Command::new(".\\output\\out.exe")
            .status()
            .unwrap()
            .code()
            .unwrap()
    )
}
