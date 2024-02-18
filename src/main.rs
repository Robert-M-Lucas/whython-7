use crate::assembler::assemble::{assemble, generate_assembly, link, link_gcc_experimental};

use crate::parser::parse::parse;
use crate::processor::processor::process;
use std::path::PathBuf;
use std::process::Command;

use std::time::Instant;

mod assembler;
mod ast;
mod basic_ast;
mod compiler;
mod custom;
mod parser;
mod processor;

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
    #[cfg(target_os = "linux")]
    {
        println!("Linking and execution might be buggy due to Linux being unsupported");
        println!("Linking (gcc)...");
        link_gcc_experimental();
        println!("Executing (wine)...");
        run_wine_experimental();
    }
}

fn run() {
    let code = Command::new(".\\output\\out.exe")
        .status()
        .unwrap()
        .code()
        .unwrap();
    println!(
        "\nExited with return code {} ({:?})",
        code,
        code % 16 == 0
    )
}

fn run_wine_experimental() {
    println!(
        "\nExited with return code {}",
        Command::new("wine")
            .args(["./output/out.exe"])
            .status()
            .unwrap()
            .code()
            .unwrap()
    )
}
