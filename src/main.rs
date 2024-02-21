use std::fs;
use crate::assembler::assemble::{assemble, generate_assembly, link, link_gcc_experimental};

use crate::parser::parse::parse;
use crate::processor::processor::process;
use std::path::PathBuf;
use std::process::Command;
use clap::Parser;

use std::time::Instant;
use b_box::b;

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

/// Compiler for Whython files (.why)
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Main input file
    #[arg(short, long, default_value = "main.why")]
    input: String,
    /// Output files name without extension
    /// Main input file
    #[arg(short, long, default_value = "out")]
    output: String
}

fn main() {
    let args = Args::parse();

    let mut asts = Vec::new();
    print!("Parsing...");
    time!(
        if let Err(e) = parse(PathBuf::from(&args.input), &mut asts) {
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
    time!(generate_assembly(&args.output, functions));
    print!("Assembling (NASM)...");
    time!(assemble(&args.output));
    #[cfg(target_os = "windows")]
    {
        println!("Linking (MSVC - link.exe)...");
        time!(link(&args.output));
        println!("Executing...");
        time!(run(&args.output));
    }
    #[cfg(target_os = "linux")]
    {
        println!("Linking and execution might be buggy due to Linux being unsupported");
        println!("Linking (gcc)...");
        time!(link_gcc_experimental(&args.output));
        println!("Executing (wine)...");
        time!(run_wine_experimental(&args.output));
    }
}

fn run(output: &str) {
    let full = fs::canonicalize(format!("{output}.exe")).unwrap();
    let code = Command::new(full)
        .status()
        .unwrap()
        .code()
        .unwrap();
    println!(
        "\nExited with return code {}",
        code,
    )
}

fn run_wine_experimental(output: &str) {
    let full = fs::canonicalize(format!("{output}.exe")).unwrap();
    println!(
        "\nExited with return code {}",
        Command::new("wine")
            .args([full])
            .status()
            .unwrap()
            .code()
            .unwrap()
    )
}
