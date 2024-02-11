use std::fs;
use std::path::PathBuf;
use std::process::Command;
use crate::compiler::compile_functions::Function;


pub fn generate_assembly(output: &PathBuf, functions: Vec<Box<dyn Function>>) {
    let mut out = String::from("    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text\n");
    for f in functions {
        out.push('\n');
        out += &(f.get_asm());
    }

    fs::write("output\\out.asm", out).unwrap();
}

pub fn assemble() {
    Command::new("nasm")
        .args(["-f", "win64", ".\\output\\out.asm"])
        .status()
        .unwrap();
}

pub fn link() {
    Command::new("link")
        .args(["/entry:main", "/out:.\\output\\out.exe", "/SUBSYSTEM:CONSOLE", "/LARGEADDRESSAWARE:NO", ".\\output\\out.obj", ".\\libs\\kernel32.lib"])
        .status()
        .unwrap();
}