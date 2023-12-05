use crate::parser_state::ParserState;
use crate::compiler::main::tokens;
use crate::lexer_state::LexerState;
use crate::global_state::GlobalState;
use std::process::{Command, exit};
use std::fs::{write, remove_file};

pub mod main {
    pub mod tokens {
        pub enum Token {
            Output,
            Bracket,
            Quotes,
            String,
            Semicolon,
            Main,
            Ok,
            Outputln,
            Linefeed,
            INT32,
            Curly,
            Fmt,
            Decrement,
            Increment
        }

        pub static TOKENS: &'static [&'static str] = &["op", "(", ")", "\"", ";", "main[Status]", "Ok()", "}", "opln", "crt-int-:", "{"];
        pub static OPERATORS: &'static [&'static str] = &["++", "--"];
    }
}
    pub fn lexer(code: &str, lexer_state: &mut LexerState, global_state: &mut GlobalState) {
        let source_code = code.chars().collect::<Vec<char>>();

        for i in 0..code.len() {
            lexer_state.current_command.push(source_code[i]);
            if source_code[i].is_whitespace() && !lexer_state.string_operation {
                if !lexer_state.current_command.trim().is_empty() { continue; } else {
                    lexer_state.current_command.clear();
                    continue;
                }
            } else if lexer_state.is_variable_name {
                if source_code[i] != ':' { lexer_state.variable_name.push(source_code[i]); } else {
                    global_state.variable_names.push(lexer_state.variable_name.clone());
                    lexer_state.variable_name.clear();
                    lexer_state.is_variable_name = false;
                    lexer_state.is_number = true;
                }
            } else if lexer_state.is_number {
                if source_code[i] != ';' {
                    lexer_state.current_number.push(source_code[i]);
                } else {
                    lexer_state.current_number.parse::<i32>().expect("Error");
                    lexer_state.operations.push(Some(tokens::Token::Semicolon));
                    global_state.variable_value.push(lexer_state.current_number.parse().unwrap());
                    lexer_state.current_number.clear();
                    lexer_state.is_number = false;
                    lexer_state.current_command.clear();
                }
            } else if lexer_state.string_operation && source_code[i] != '"' {
                if let Some(&Some(tokens::Token::String)) = lexer_state.operations.last() {
                    lexer_state.current_string_content.push(source_code[i]);
                    lexer_state.current_command.clear();
                } else {
                    lexer_state.operations.push(Some(tokens::Token::String));
                    lexer_state.current_string_content.push(source_code[i]);
                    lexer_state.current_command.clear();
                }
            } else if lexer_state.fmt_operation && source_code[i] != '}' {
                if let Some(&Some(tokens::Token::Fmt)) = lexer_state.operations.last() {
                    lexer_state.current_fmt_variable_name.push(source_code[i]);
                    lexer_state.current_command.clear();
                } else {
                    lexer_state.operations.push(Some(tokens::Token::Fmt));
                    lexer_state.current_string_content.push(source_code[i]);
                    lexer_state.current_command.clear();
                }
            } else if tokens::OPERATORS.iter().any(|&s| lexer_state.current_command.contains(s)) {
                if lexer_state.current_command.ends_with("++") {
                    if let Some(index) = lexer_state.current_command.rfind("++") { global_state.increment_names.push(lexer_state.current_command[..index].to_string()); }
                    lexer_state.operations.push(Some(tokens::Token::Increment));
                    lexer_state.current_command.clear();
                } else if lexer_state.current_command.ends_with("--") {
                    if let Some(index) = lexer_state.current_command.rfind("--") { global_state.decrement_names.push(lexer_state.current_command[..index].to_string()); }
                    lexer_state.operations.push(Some(tokens::Token::Decrement));
                    lexer_state.current_command.clear();
            }
            } else if tokens::TOKENS.iter().any(|&s| s.contains(&*lexer_state.current_command)) {
                match lexer_state.current_command.as_str() {
                    "crt-int-" => {
                        lexer_state.operations.push(Some(tokens::Token::INT32));
                        lexer_state.current_command.clear();
                        lexer_state.is_variable_name = true;
                    }
                    "op" => {
                        if source_code[i + 1] != 'l' {
                            lexer_state.operations.push(Some(tokens::Token::Output));
                            lexer_state.current_command.clear();
                        } else { continue }
                    }
                    "{" => {
                        lexer_state.operations.push(Some(tokens::Token::Curly));
                        lexer_state.current_command.clear();
                        lexer_state.count_curly += 1;
                        if lexer_state.count_curly == 1 {
                            lexer_state.fmt_operation = true;
                        } else {
                            global_state.fmt_string.push(lexer_state.current_fmt_variable_name.clone());
                            lexer_state.current_fmt_variable_name.clear();
                            lexer_state.fmt_operation = false;
                            lexer_state.count_curly = 0;
                        }
                    }
                    "\"" => {
                        lexer_state.operations.push(Some(tokens::Token::Quotes));
                        lexer_state.current_command.clear();
                        lexer_state.count_quotes += 1;
                        if lexer_state.count_quotes == 1 {
                            lexer_state.string_operation = true;
                        } else {
                            global_state.strings_contents.push(lexer_state.current_string_content.clone());
                            lexer_state.current_string_content.clear();
                            lexer_state.string_operation = false;
                            lexer_state.count_quotes = 0;
                        }
                    }
                    "opln" => {
                        lexer_state.operations.push(Some(tokens::Token::Outputln));
                        lexer_state.operations.push(Some(tokens::Token::Linefeed));
                        lexer_state.current_command.clear();
                    }
                    "main[Status]" => {
                        lexer_state.operations.push(Some(tokens::Token::Main));
                        lexer_state.current_command.clear();
                    }
                    "(" => {
                        lexer_state.operations.push(Some(tokens::Token::Bracket));
                        lexer_state.current_command.clear();
                    }
                    ")" => {
                        lexer_state.operations.push(Some(tokens::Token::Bracket));
                        lexer_state.current_command.clear();
                    }
                    ";" => {
                        lexer_state.operations.push(Some(tokens::Token::Semicolon));
                        lexer_state.current_command.clear();
                    }
                    "Ok()" => {
                        lexer_state.operations.push(Some(tokens::Token::Ok));
                        lexer_state.current_command.clear();
                    }
                    _ => continue,
                }
            }
        }
    }

    pub fn parser(operations: &Vec<Option<tokens::Token>>, parser_state: &mut ParserState, global_state: &mut GlobalState) {
        while parser_state.current_index < operations.len() {
            if let Some(oper) = &operations[parser_state.current_index] {
                if !global_state.variable_names.is_empty() && global_state.variable_names.len() - 1 >= parser_state.variable_name_index { parser_state.fmt_name_variable = global_state.variable_names[parser_state.variable_name_index].clone(); }
                if !global_state.variable_value.is_empty() && global_state.variable_value.len() - 1 >= parser_state.number_index { parser_state.fmt_variable_value = global_state.variable_value[parser_state.number_index]; }
                if !global_state.increment_names.is_empty() && global_state.increment_names.len() - 1 >= parser_state.increment_index { parser_state.fmt_increment_variable_name = global_state.increment_names[parser_state.increment_index].clone(); }
                if !global_state.decrement_names.is_empty() && global_state.decrement_names.len() - 1 >= parser_state.decrement_index { parser_state.fmt_decrement_variable_name = global_state.decrement_names[parser_state.decrement_index].clone(); }
                match oper {
                    tokens::Token::INT32 => {
                        parser_state.current_operation.push_str(&*format!("let mut {0}: i32 = {1}", parser_state.fmt_name_variable, parser_state.fmt_variable_value));
                        parser_state.variable_name_index += 1;
                        parser_state.number_index += 1;
                    }
                    tokens::Token::Ok => {
                        parser_state.current_operation.push_str("print!(\"\nProcess finished with exit status Ok\");");
                        parser_state.current_operation.push('}');
                        if parser_state.current_index != operations.len() - 1 {
                            eprintln!("Process finished with exit status Err");
                            exit(1);
                        }
                    }
                    tokens::Token::Bracket => {
                        if parser_state.bracket_is_used == false {
                            parser_state.current_operation.push('(');
                            parser_state.bracket_is_used = true;
                        } else {
                            parser_state.bracket_is_used = false;
                            parser_state.current_operation.push(')');
                        }
                    }
                    tokens::Token::Quotes => {
                        parser_state.current_operation.push('"');
                        if parser_state.linefeed {
                            parser_state.current_operation.push_str("\n");
                            parser_state.linefeed = false;
                        }
                    }
                    tokens::Token::Decrement => {
                        parser_state.current_operation.push_str(&*format!("{0} -= 1;", parser_state.fmt_decrement_variable_name));
                        parser_state.decrement_index += 1;
                    }
                    tokens::Token::Increment => {
                        parser_state.current_operation.push_str(&*format!("{0} += 1;", parser_state.fmt_increment_variable_name));
                        parser_state.increment_index += 1;
                    }
                    tokens::Token::Fmt => {
                        parser_state.current_operation.push_str(&global_state.fmt_string[parser_state.curly_index]);
                        parser_state.curly_index += 1;
                    }
                    tokens::Token::String => {
                        parser_state.current_operation.push_str(&global_state.strings_contents[parser_state.string_index]);
                        parser_state.string_index += 1;
                    }
                    tokens::Token::Curly => parser_state.current_operation.push('{'),
                    tokens::Token::Main => parser_state.current_operation.push_str("fn main(){ \n"),
                    tokens::Token::Output => parser_state.current_operation.push_str("print!"),
                    tokens::Token::Outputln => parser_state.current_operation.push_str("print!"),
                    tokens::Token::Linefeed => parser_state.linefeed = true,
                    tokens::Token::Semicolon => parser_state.current_operation.push_str(";\n"),
                }
            }
            parser_state.current_index += 1;
        }
    }

    pub fn compiler(code: &str) {
        write("output.rs", code).expect("Failed to write file");

        let output = Command::new("rustc")
            .args(&["output.rs"])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let run_output = Command::new("./output")
                .output()
                .expect("Failed to execute command");
            eprintln!("{}", String::from_utf8_lossy(&run_output.stdout));
            remove_file("output.rs").expect("Failed to remove file");
            exit(0);
        } else {
            remove_file("output.rs").expect("Failed to remove file");
            eprintln!("Process finished with exit status Err");
            exit(1);
        }
}


