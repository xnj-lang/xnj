mod parser;

use std::env::args;
use std::io::{Read};
use std::fs::OpenOptions;
use std::process::exit;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Fatal Error!\nExample: compiler main.xnj");
        exit(1);
    }
    let file_name = &args[1];
    if is_xnj_ext(file_name) {
        let mut compiler = OpenOptions::new()
            .read(true)
            .open(&file_name)
            .expect("Error");
        let mut file_content = String::new();
        compiler.read_to_string(&mut file_content).expect("Error reading file");
        parser::main::parse(file_content.as_str());
    }
    else {
        eprintln!("Fatal Error!\nMust be .xnj extension!");
        exit(1);
    }
}

fn is_xnj_ext(file_path: &str) -> bool {
    let path = Path::new(file_path);
    if let Some(extension) = path.extension() {
        return extension == "xnj";
    }
    false
}
