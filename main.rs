mod parser;

use std::env::args;
use std::io::{Read};
use std::fs::OpenOptions;
use std::process::exit;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Example: ./compiler.exe <filename>");
        exit(1);
    }
    let file_name = &args[1];
    let mut compiler = OpenOptions::new()
        .read(true)
        .open(&file_name)
        .expect("Error");
    let mut file_content = String::new();
    compiler.read_to_string(&mut file_content).expect("Error reading file");
    parser::parse::parse(file_content.as_str());
}