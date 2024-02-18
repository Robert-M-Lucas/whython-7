use crate::compiler::compile_functions::Function;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn generate_assembly(_output: &PathBuf, functions: Vec<Box<dyn Function>>) {
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

    if !PathBuf::from("output").as_path().is_dir() {
        fs::create_dir("output").expect("Failed to create output folder");
    }
    fs::write("output/out.asm", out).expect("Failed to write assembly to file");
}

pub fn assemble() {
    if !Command::new("nasm")
        .args(["-f", "win64", "./output/out.asm"])
        .status()
        .unwrap()
        .success()
    {
        panic!("NASM assembler step failed");
    }
}

pub fn link() {
    if !Command::new("link")
        .args([
            "/entry:main",
            "/out:.\\output\\out.exe",
            "/SUBSYSTEM:CONSOLE",
            // "/LARGEADDRESSAWARE:NO",
            ".\\output\\out.obj",
            ".\\libs\\kernel32.lib",
        ])
        .status()
        .unwrap()
        .success()
    {
        panic!("MSVC linking step failed");
    }
}

pub fn link_gcc_experimental() {
    if !Command::new("x86_64-w64-mingw32-gcc")
        .args([
            "./output/out.obj",
            "./libs/kernel32.lib",
            "-o",
            "./output/out.exe"
        ])
        .status()
        .unwrap()
        .success()
    {
        panic!("gcc linking step failed");
    }
}