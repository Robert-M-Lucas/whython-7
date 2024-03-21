use std::fs;
use std::process::Command;

#[cfg(target_os = "windows")]
pub fn run(output: &str) {
    let full = fs::canonicalize(format!("{output}.exe")).unwrap();
    let code = Command::new(full).status().unwrap().code().unwrap();
    println!("\nExited with return code {}", code,)
}

#[cfg(target_os = "linux")]
pub fn run_wine_experimental(output: &str) {
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

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "linux")]
pub fn link_gcc_experimental(output: &str) {
    if !Command::new("x86_64-w64-mingw32-gcc")
        .args([
            format!("{output}.obj").as_str(),
            "./libs/kernel32.lib",
            "-o",
            format!("{output}.exe").as_str(),
        ])
        .status()
        .unwrap()
        .success()
    {
        panic!("gcc linking step failed");
    }
}
