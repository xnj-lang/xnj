pub mod main {
    use std::process::Command;
    use std::fs::{write, remove_file, metadata};
    use std::process::exit;

    enum Macros {
        Output,
        Bracket,
        Quotes,
        String,
        Semicolon,
        Main,
        Ok,
        //Curly,
    }

    pub fn parse(code: &str) {
        if metadata("run.rs").is_ok() {
            remove_file("run.rs").expect("Failed to remove file");
        }
        let macros_vec: Vec<&str> = vec!["output", "(", ")", "\"", ";", "main[Status]", "Ok()", "}"];
        let code = code.chars().collect::<Vec<char>>();
        let mut command = String::new();
        let mut opers: Vec<Option<Macros>> = Vec::new();
        let mut string_oper = false;
        let mut count = 0;
        let mut curr_string = String::new();

        for char in code {
            command.push(char);
            if char.is_whitespace() {
                if !command.trim().is_empty() {
                    continue;
                } else {
                    command.clear();
                    continue;
                }
            } else if string_oper && char != '"' {
                if let Some(&Some(Macros::String)) = opers.last() {
                    curr_string.push(char);
                    command.clear();
                } else {
                    opers.push(Some(Macros::String));
                    curr_string.push(char);
                    command.clear();
                }
            } else if macros_vec.iter().any(|&s| s.contains(&*command)) {
                match command.as_str() {
                    "output" => {
                        opers.push(Some(Macros::Output));
                        command.clear();
                    }
                    "main[Status]" => {
                        opers.push(Some(Macros::Main));
                        command.clear();
                    }
                    "(" => {
                        opers.push(Some(Macros::Bracket));
                        command.clear();
                    }
                    ")" => {
                        opers.push(Some(Macros::Bracket));
                        command.clear();
                    }
                    ";" => {
                        opers.push(Some(Macros::Semicolon));
                        command.clear();
                    }
                    "Ok()" => {
                        opers.push(Some(Macros::Ok));
                        command.clear();
                    }
                    "\"" => {
                        opers.push(Some(Macros::Quotes));
                        command.clear();
                        count += 1;
                        if count == 1 {
                            string_oper = true;
                        } else {
                            string_oper = false;
                            count = 0;
                        }
                    }
                    _ => continue,
                }
            }
        }

        let mut curr_oper = String::new();
        let mut used = false;
        let mut current_index = 0;

        let total_operators = opers.len();

        while current_index < total_operators {
            if let Some(oper) = &opers[current_index] {
                match oper {
                    Macros::Bracket => {
                        if used == false {
                            curr_oper.push('(');
                            used = true;
                        } else {
                            used = false;
                            curr_oper.push(')');
                        }
                    }
                    Macros::String => {
                        curr_oper.push_str(&curr_string);
                        curr_string.clear();
                    }
                    Macros::Main => curr_oper.push_str("fn main(){ \n"),
                    Macros::Output => curr_oper.push_str("print!"),
                    Macros::Quotes => curr_oper.push('"'),
                    Macros::Semicolon => curr_oper.push_str(";\n"),
                    Macros::Ok => {
                        curr_oper.push_str("print!(\"\nProcess finished with exit status Ok\");");
                        curr_oper.push('}');
                        if current_index != total_operators - 1 {
                            eprintln!("Process finished with exit status Err");
                            exit(1);
                        }
                    }
                    //Macros::Curly => curr_oper.push('}')
                }
            }
            current_index += 1;
        }

        write("run.rs", curr_oper).expect("Failed to write file");
        let output = Command::new("rustc")
            .args(&["run.rs"])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let run_output = Command::new("./run")
                .output()
                .expect("Failed to execute command");
            remove_file("run.rs").expect("Failed to remove file");
            eprintln!("{}", String::from_utf8_lossy(&run_output.stdout));
            exit(1);
        } else {
            //eprintln!("Compilation failed: {:?}", String::from_utf8_lossy(&output.stderr));
            remove_file("run.rs").expect("Failed to remove file");
            eprintln!("Process finished with exit status Err");
            exit(1);
        }
    }
}
