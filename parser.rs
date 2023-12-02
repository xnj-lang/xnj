pub mod main {
    use std::process::{Command, exit};
    use std::fs::{write, remove_file, metadata};

    enum Macros {
        Output,
        Bracket,
        Quotes,
        String,
        Semicolon,
        Main,
        Ok,
        Outputln,
        Linefeed
    }

    pub fn parse(code: &str) {
        if metadata("run.rs").is_ok() { remove_file("run.rs").expect("Failed to remove file");}

        let macros_vec: Vec<&str> = vec!["op", "(", ")", "\"", ";", "main[Status]", "Ok()", "}", "opln"];
        let code = code.chars().collect::<Vec<char>>();
        let mut opers: Vec<Option<Macros>> = Vec::new();
        let mut strings = Vec::new();

        let mut command = String::new();
        let mut curr_string = String::new();
         let mut curr_oper = String::new();

        let mut count = 0;

        let mut string_oper = false;
        let mut used = false;
        let mut lf = false;
        let mut current_index = 0;
        let mut stringindex = 0;

        for i in 0..code.len() {
            command.push(code[i]);
            if code[i].is_whitespace() && !string_oper {
                if !command.trim().is_empty() { continue; }
                else {
                    command.clear();
                    continue;
                }
            }
            else if string_oper && code[i] != '"' {
                if let Some(&Some(Macros::String)) = opers.last() {
                    curr_string.push(code[i]);
                    command.clear();
                } else {
                    opers.push(Some(Macros::String));
                    curr_string.push(code[i]);
                    command.clear();
                }
            }
            else if macros_vec.iter().any(|&s| s.contains(&*command)) {
                match command.as_str() {
                    "op" => {
                        if code[i+1] != 'l' {
                            opers.push(Some(Macros::Output));
                            command.clear();
                        }
                        else{ continue }
                    }
                    "\"" => {
                        opers.push(Some(Macros::Quotes));
                        command.clear();
                        count += 1;
                        if count == 1 {
                            string_oper = true;
                        } else {
                            strings.push(curr_string.clone());
                            curr_string.clear();
                            string_oper = false;
                            count = 0;
                        }
                    }
                    "opln" => {
                        opers.push(Some(Macros::Outputln));
                        opers.push(Some(Macros::Linefeed));
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
                    _ => continue,
                }
            }
        }


        while current_index < opers.len() {
            if let Some(oper) = &opers[current_index] {
                match oper {
                    Macros::Ok => {
                        curr_oper.push_str("print!(\"\nProcess finished with exit status Ok\");");
                        curr_oper.push('}');
                        if current_index != opers.len() - 1 {
                            eprintln!("Process finished with exit status Err");
                            exit(1);
                        }
                    }
                    Macros::Bracket => {
                        if used == false {
                            curr_oper.push('(');
                            used = true;
                        } else {
                            used = false;
                            curr_oper.push(')');
                        }
                    }
                    Macros::Quotes => {
                        curr_oper.push('"');
                        if lf{
                            curr_oper.push_str("\n");
                            lf = false;
                        }
                    }
                    Macros::String => {
                        curr_oper.push_str(&strings[stringindex]);
                        stringindex+=1;
                    }
                    Macros::Main => curr_oper.push_str("fn main(){ \n"),
                    Macros::Output => curr_oper.push_str("print!"),
                    Macros::Outputln => curr_oper.push_str("print!"),
                    Macros::Linefeed => lf = true,
                    Macros::Semicolon => curr_oper.push_str(";\n"),
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
            eprintln!("{}", String::from_utf8_lossy(&run_output.stdout));
            exit(1);
        } else {
            remove_file("run.rs").expect("Failed to remove file");
            eprintln!("Process finished with exit status Err");
            exit(1);
        }
    }
}
