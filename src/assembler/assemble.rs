use std::fs;
use std::path::PathBuf;
use std::process::Command;
use crate::assembler::default::get_function_name;
use crate::processor::function_processor::Function;



pub fn generate_assembly(output: &PathBuf, functions: Vec<Box<dyn Function>>) {
    let mut out = String::from("\tglobal main\n\textern ExitProcess\n\tsection .text\n");
    for f in functions {
        out += &(f.get_asm());
    }

    fs::write("output\\out.asm", out).unwrap();
}

pub fn assemble(output: &PathBuf) {
    Command::new("nasm")
        .args(["-f", "win64", ".\\output\\out.asm"])
        .status()
        .unwrap();

    Command::new("link")
        .args(["/entry:main", "/out:.\\output\\out.exe", ".\\output\\out.obj", ".\\libs\\kernel32.lib"])
        .status()
        .unwrap();
}