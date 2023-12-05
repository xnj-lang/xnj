mod compiler;
mod data;

use std::env::args;
use std::io::Read;
use std::fs::{OpenOptions, metadata, remove_file};
use std::process::exit;
use std::path::Path;
use crate::data::{global_state, parser_state, lexer_state};
use crate::compiler::{lexer, parser, compiler};
pub use crate::lexer_state::LexerState;
pub use crate::parser_state::ParserState;
pub use crate::global_state::GlobalState;

fn main() {
    if metadata("output.rs").is_ok() { remove_file("output.rs").expect("Failed to remove file");}
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Fatal Error!\nExample: xnjc main.xnj");
        exit(1);
    }
    let file_name = &args[1];
    if is_xnj_ext(file_name) {
        let mut sourse_code = OpenOptions::new()
            .read(true)
            .open(&file_name)
            .expect("Error");
        let mut file_content = String::new();
        sourse_code.read_to_string(&mut file_content).expect("Error reading file");
        let (mut lexer_vars, mut parser_vars, mut global_vars) = (LexerState::default(), ParserState::default(), GlobalState::default());
        lexer(file_content.as_str(), &mut lexer_vars, &mut global_vars);
        parser(&lexer_vars.operations, &mut parser_vars, &mut global_vars);
        compiler(&parser_vars.current_operation);
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
