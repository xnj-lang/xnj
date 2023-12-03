pub mod main {
    use std::process::{Command, exit};
    use std::fs::{write, remove_file, metadata};

    enum Token {
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

    pub fn parse(code: &str) {
        if metadata("output.rs").is_ok() { remove_file("output.rs").expect("Failed to remove file");}

        let tokens: Vec<&str> = vec!["op", "(", ")", "\"", ";", "main[Status]", "Ok()", "}", "opln", "crt-int-:", "{"];
        let operators: Vec<&str> = vec!["++", "--"];

        let source_code = code.chars().collect::<Vec<char>>();

        let mut operations: Vec<Option<Token>> = Vec::new();

        let mut strings_contents = Vec::new();
        let mut variable_names = Vec::new();
        let mut variable_value: Vec<i32> = Vec::new();
        let mut fmt_string: Vec<String> = Vec::new();
        let mut increment_names: Vec<String> = Vec::new();
        let mut decrement_names: Vec<String> = Vec::new();

        let mut current_command = String::new();
        let mut current_string_content = String::new();
        let mut current_operation = String::new();
        let mut current_fmt_variable_name = String::new();
        let mut variable_name = String::new();
        let mut current_number = String::new();
        let mut fmt_name_variable = "".to_string();
        let mut fmt_increment_variable_name = "".to_string();
        let mut fmt_decrement_variable_name = "".to_string();

        let mut count_quotes = 0;
        let mut count_curly = 0;
        let mut fmt_variable_value = 0;

        let mut string_operation = false;
        let mut bracket_is_used = false;
        let mut linefeed = false;
        let mut is_variable_name = false;
        let mut is_number = false;
        let mut fmt_operation = false;

        let mut increment_index = 0;
        let mut decrement_index = 0;
        let mut number_index = 0;
        let mut variable_name_index = 0;
        let mut current_index = 0;
        let mut string_index = 0;
        let mut curly_index = 0;

        for i in 0..code.len() {
            current_command.push(source_code[i]);
            if source_code[i].is_whitespace() && !string_operation {
                if !current_command.trim().is_empty() { continue; }
                else {
                    current_command.clear();
                    continue;
                }
            }
            else if is_variable_name{
                if source_code[i] != ':'{ variable_name.push(source_code[i]); }
                else {
                    variable_names.push(variable_name.clone());
                    variable_name.clear();
                    is_variable_name = false;
                    is_number = true;
                }
            }
            else if is_number{
                if source_code[i] != ';'{
                    current_number.push(source_code[i]);
                }
                else{
                    current_number.parse::<i32>().expect("Error");
                    operations.push(Some(Token::Semicolon));
                    variable_value.push(current_number.parse().unwrap());
                    current_number.clear();
                    is_number = false;
                    current_command.clear();
                }
            }
            else if string_operation && source_code[i] != '"' {
                if let Some(&Some(Token::String)) = operations.last() {
                    current_string_content.push(source_code[i]);
                    current_command.clear();
                } else {
                    operations.push(Some(Token::String));
                    current_string_content.push(source_code[i]);
                    current_command.clear();
                }
            }
            else if fmt_operation && source_code[i] != '}' {
                if let Some(&Some(Token::Fmt)) = operations.last() {
                    current_fmt_variable_name.push(source_code[i]);
                    current_command.clear();
                } else {
                    operations.push(Some(Token::Fmt));
                    current_string_content.push(source_code[i]);
                    current_command.clear();
                }
            }
            else if operators.iter().any(|&s| current_command.contains(s)) {
                if current_command.ends_with("++") {
                    if let Some(index) = current_command.rfind("++") { increment_names.push(current_command[..index].to_string()); }
                    operations.push(Some(Token::Increment));
                    current_command.clear();
                }
                else if current_command.ends_with("--") {
                    if let Some(index) = current_command.rfind("--") { decrement_names.push(current_command[..index].to_string()); }
                    operations.push(Some(Token::Decrement));
                    current_command.clear();
                }
            }
            else if tokens.iter().any(|&s| s.contains(&*current_command)) {
                match current_command.as_str() {
                    "crt-int-" =>{
                        operations.push(Some(Token::INT32));
                        current_command.clear();
                        is_variable_name = true;
                    }
                    "op" => {
                        if source_code[i+1] != 'l' {
                            operations.push(Some(Token::Output));
                            current_command.clear();
                        }
                        else{ continue }
                    }
                    "{" => {
                        operations.push(Some(Token::Curly));
                        current_command.clear();
                        count_curly += 1;
                        if count_curly == 1{
                            fmt_operation = true;
                        }
                        else{
                            fmt_string.push(current_fmt_variable_name.clone());
                            current_fmt_variable_name.clear();
                            fmt_operation = false;
                            count_curly = 0;
                        }
                    }
                    "\"" => {
                        operations.push(Some(Token::Quotes));
                        current_command.clear();
                        count_quotes += 1;
                        if count_quotes == 1 {
                            string_operation = true;
                        } else {
                            strings_contents.push(current_string_content.clone());
                            current_string_content.clear();
                            string_operation = false;
                            count_quotes = 0;
                        }
                    }
                    "opln" => {
                        operations.push(Some(Token::Outputln));
                        operations.push(Some(Token::Linefeed));
                        current_command.clear();
                    }
                    "main[Status]" => {
                        operations.push(Some(Token::Main));
                        current_command.clear();
                    }
                    "(" => {
                        operations.push(Some(Token::Bracket));
                        current_command.clear();
                    }
                    ")" => {
                        operations.push(Some(Token::Bracket));
                        current_command.clear();
                    }
                    ";" => {
                        operations.push(Some(Token::Semicolon));
                        current_command.clear();
                    }
                    "Ok()" => {
                        operations.push(Some(Token::Ok));
                        current_command.clear();
                    }
                    _ => continue,
                }
            }
        }


        while current_index < operations.len() {
            if let Some(oper) = &operations[current_index] {
                if !variable_names.is_empty() && variable_names.len() - 1 >= variable_name_index { fmt_name_variable = variable_names[variable_name_index].clone(); }
                if !variable_value.is_empty() && variable_value.len() - 1 >= number_index { fmt_variable_value = variable_value[number_index]; }
                if !increment_names.is_empty() && increment_names.len() - 1 >= increment_index { fmt_increment_variable_name = increment_names[increment_index].clone(); }
                if !decrement_names.is_empty() && decrement_names.len() -1 >= decrement_index { fmt_decrement_variable_name = decrement_names[decrement_index].clone(); }
                match oper {
                    Token::INT32 => {
                        current_operation.push_str(&*format!("let mut {fmt_name_variable}: i32 = {fmt_variable_value}"));
                        variable_name_index+=1;
                        number_index+=1;
                    }
                    Token::Ok => {
                        current_operation.push_str("print!(\"\nProcess finished with exit status Ok\");");
                        current_operation.push('}');
                        if current_index != operations.len() - 1 {
                            eprintln!("Process finished with exit status Err");
                            exit(1);
                        }
                    }
                    Token::Bracket => {
                        if bracket_is_used == false {
                            current_operation.push('(');
                            bracket_is_used = true;
                        } else {
                            bracket_is_used = false;
                            current_operation.push(')');
                        }
                    }
                    Token::Quotes => {
                        current_operation.push('"');
                        if linefeed{
                            current_operation.push_str("\n");
                            linefeed = false;
                        }
                    }
                    Token::Decrement => {
                        current_operation.push_str(&*format!("{fmt_decrement_variable_name} -= 1;"));
                        decrement_index+=1;
                    }
                    Token::Increment => {
                        current_operation.push_str(&*format!("{fmt_increment_variable_name} += 1;"));
                        increment_index+=1;
                    }
                    Token::Fmt => {
                        current_operation.push_str(&fmt_string[curly_index]);
                        curly_index+=1;
                    }
                    Token::String => {
                        current_operation.push_str(&strings_contents[string_index]);
                        string_index+=1;
                    }
                    Token::Curly => current_operation.push('{'),
                    Token::Main => current_operation.push_str("fn main(){ \n"),
                    Token::Output => current_operation.push_str("print!"),
                    Token::Outputln => current_operation.push_str("print!"),
                    Token::Linefeed => linefeed = true,
                    Token::Semicolon => current_operation.push_str(";\n"),
                }
            }
            current_index += 1;
        }

        write("output.rs", current_operation).expect("Failed to write file");
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
            exit(1);
        } else {
            remove_file("output.rs").expect("Failed to remove file");
            eprintln!("Process finished with exit status Err");
            exit(1);
        }
    }
}
