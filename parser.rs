pub mod main{
    use std::process::Command;
    use std::fs::write;
    use std::fs::remove_file;
    use std::fs::metadata;

    enum Macros {
        Output,
        Bracket,
        Quotes,
        String,
        Semicolon,
        Main,
        Ok
    }
    pub fn parse(code: &str){
        if metadata("run.rs").is_ok() {
            remove_file("run.rs").expect("Failed to remove file");
        }
        let macros_vec: Vec<&str> = vec!["output", "(", ")", "\"", ";", "main{", "Ok()"];
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
                }
                else{
                    command.clear();
                    continue;
                }
            }
            else if string_oper && char != '"' {
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
                    "main{" => {
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
                        count+=1;
                        if count == 1 {
                            string_oper = true;
                        }
                        else{
                            string_oper = false;
                            count = 0;
                        }
                    }
                    _ => continue
                }
            }
        }

        let mut curr_oper = String::new();
        let mut used = false;
        for operator in opers {
            if let Some(oper) = operator {
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
                    Macros::Ok => curr_oper.push_str("print!(\"\nProcess finished with exit status Ok\")")
                }
            }
        }
        curr_oper.push('}');
        write("run.rs", curr_oper).expect("Failed to write file");
        let output = Command::new("rustc")
            .args(&["run.rs"])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let run_output = Command::new("./run")
                .output()
                .expect("Failed to execute command");
            println!("{}", String::from_utf8_lossy(&run_output.stdout));
        } else {
            eprintln!("Compilation failed: {:?}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
