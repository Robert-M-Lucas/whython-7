use crate::compiler::compile_functions::Function;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn generate_assembly(output: &str, functions: Vec<Box<dyn Function>>) {
    let mut out = String::from(
        "    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text\n",
    );
    for f in functions {
        out += "\r\n";
        out += &(f.get_asm());
    }

    fs::write(format!("{output}.asm"), out).expect("Failed to write assembly to file");
}

pub fn assemble(output: &str) {
    if !Command::new("nasm")
        .args(["-f", "win64", format!("{output}.asm").as_str()])
        .status()
        .unwrap()
        .success()
    {
        panic!("NASM assembler step failed");
    }
}

pub fn link(output: &str) {
    if !Command::new("link")
        .args([
            "/entry:main",
            format!("/out:{output}.exe").as_str(),
            "/SUBSYSTEM:CONSOLE",
            // "/LARGEADDRESSAWARE:NO",
            format!("{output}.obj").as_str(),
            ".\\libs\\kernel32.lib",
        ])
        .status()
        .unwrap()
        .success()
    {
        panic!("MSVC linking step failed");
    }
}

pub fn link_gcc_experimental(output: &str) {

    if !Command::new("x86_64-w64-mingw32-gcc")
        .args([
            format!("{output}.asm").as_str(),
            "./libs/kernel32.lib",
            "-o",
            format!("{output}.exe").as_str()
        ])
        .status()
        .unwrap()
        .success()
    {
        panic!("gcc linking step failed");
    }
}