use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use whython_7::root::main_args;
use whython_7::root::Args;

#[test]
fn tests() {
    fs::create_dir_all("tests/temp").unwrap();
    
    for path in fs::read_dir("tests/inputs").unwrap() {
        let mut out_path = PathBuf::from("tests/outputs");
        out_path.push(path.as_ref().unwrap().file_name());
        compare(path.unwrap().path().into_os_string().into_string().unwrap(), out_path.into_os_string().into_string().unwrap());
    }
}

fn compare(in_path: String, out_path: String) {
    assert!(main_args(Args { input: in_path, output: String::from("tests/temp/out"), build: true }));
    
    let result = Command::new("tests/temp/out.exe")
        .output().unwrap();
    
    assert!(result.status.success());
    
    let result = String::from_utf8(result.stdout).unwrap().replace('\0', "").replace("\r\n", "\n");
    
    assert_eq!(result, fs::read_to_string(out_path).unwrap().replace("\r\n", "\n"));
}