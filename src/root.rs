#[cfg(target_os = "windows")]
use crate::root::assembler::assemble::link;
#[cfg(target_os = "linux")]
use crate::root::assembler::assemble::link_gcc_experimental;
use crate::root::assembler::assemble::{assemble, generate_assembly};
use std::fs;

use crate::root::parser::parse::parse;
use crate::root::processor::processor::process;
use clap::Parser;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;
use crate::time;

mod assembler;
mod ast;
mod basic_ast;
mod compiler;
mod custom;
mod parser;
mod processor;
mod utils;

// TODO: Handle circular imports

/// Compiler for Whython files (.why)
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Main input file
    #[arg(short, long, default_value = "main.why")]
    pub input: String,
    /// Output files name without extension
    /// Main input file
    #[arg(short, long, default_value = "out")]
    pub output: String,
    /// Only build - don't run
    #[arg(short, long)]
    pub build: bool
}

pub fn main() {
    // assemble("out");
    // link("out");
    // run("out");
    // return;

    let args = Args::parse();
    main_args(args);
}

pub fn main_args(args: Args) -> bool {
    let mut asts = Vec::new();
    print!("Parsing...");
    time!(
        if let Err(e) = parse(PathBuf::from(&args.input), &mut asts) {
            println!("\n{}", e);
            return false;
        }
    );

    print!("Processing...");
    time!(
        let functions = match process(asts) {
            Err(e) => {
                println!("\n{}", e);
                return false;
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
        if args.build {
            println!("Skipping execution")
        }
        else {
            println!("Executing...");
            time!(run(&args.output));
        }
    }
    #[cfg(target_os = "linux")]
    {
        println!("Compilation and execution on Linux may be buggy!");
        println!("Linking (gcc)...");
        time!(link_gcc_experimental(&args.output));
        if args.build {
            println!("Skipping execution")
        }
        else {
            println!("Executing (wine)...");
            time!(run_wine_experimental(&args.output));
        }
    }
    println!("Done!");
    true
}

#[cfg(target_os = "windows")]
fn run(output: &str) {
    let full = fs::canonicalize(format!("{output}.exe")).unwrap();
    let code = Command::new(full).status().unwrap().code().unwrap();
    println!("\nExited with return code {}", code,)
}

#[cfg(target_os = "linux")]
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
