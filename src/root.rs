use crate::root::assembler::assemble::generate_assembly;
use crate::root::parser::parse::parse;
use crate::root::processor::processor::process;
use crate::time;
use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;
use runner::assemble;
use crate::root::utils::AnyError;

#[cfg(target_os = "windows")]
use runner::link;
#[cfg(target_os = "windows")]
use crate::root::runner::run;

#[cfg(target_os = "linux")]
use runner::link_gcc_experimental;
#[cfg(target_os = "linux")]
use crate::root::runner::run_wine_experimental;

mod assembler;
mod ast;
mod basic_ast;
mod compiler;
mod custom;
mod parser;
mod processor;
mod utils;
mod runner;

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
    pub build: bool,
}

pub fn main() {
    // assemble("out");
    // link("out");
    // run("out");
    // return;
    
    let args = Args::parse();
    let _ = main_args(args);
}

pub fn main_args(args: Args) -> Result<(), AnyError> {
    let mut asts = Vec::new();
    let mut files_followed = Vec::new();
    print!("Parsing...");
    time!(
        if let Err(e) = parse(PathBuf::from(&args.input), &mut asts, &mut files_followed) {
            println!("\n{}", e);
            return Err(e.into());
        }
    );

    print!("Processing...");
    time!(
        let functions = match process(asts) {
            Err(e) => {
                println!("\n{}", e);
                return Err(e.into());
            }
            Ok(functions) => functions
        };
    );

    print!("Compiling...");
    time!(generate_assembly(&args.output, functions););
    
    print!("Assembling (NASM)...");
    time!(
        if assemble(&args.output).is_err() {
            return Err(AnyError::Other);
        }
    );
    
    #[cfg(target_os = "windows")]
    {
        println!("Linking (MSVC - link.exe)...");
        time!(
            if link(&args.output).is_err() {
                return Err(AnyError::Other);
            }
        );
        if args.build {
            println!("Skipping execution")
        } else {
            println!("Executing...");
            run(&args.output);
        }
    }
    #[cfg(target_os = "linux")]
    {
        println!("Compilation and execution on Linux may be buggy!");
        println!("Linking (gcc)...");
        time!(
            let res = link_gcc_experimental(&args.output);
            if res.is_err() {
                return Err(AnyError::Other);
            }
        );
        
        if args.build {
            println!("Skipping execution")
        } else {
            println!("Executing (wine)...");
            if run_wine_experimental(&args.output).is_err() {
                return Err(AnyError::Other);
            }
        }
    }
    
    println!("Done!");
    Ok(())
}
