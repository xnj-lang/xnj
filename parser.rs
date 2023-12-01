pub mod parse{
    enum Macros {
        Output,
        Bracket,
        Quotes,
        String
    }
    pub fn parse(code: &str){
        let macros_vec: Vec<&str> = vec!["output", "(", ")", "\""];
        let code = code.chars().collect::<Vec<char>>();
        let mut command = String::new();
        let mut opers: Vec<Option<Macros>> = Vec::new();
        let mut string_oper = false;
        let mut count = 0;
        let mut curr_string = String::new();

        for char in code {
            command.push(char);

            if string_oper && char != '"' {
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
                    "(" => {
                        opers.push(Some(Macros::Bracket));
                        command.clear();
                    }
                    ")" => {
                        opers.push(Some(Macros::Bracket));
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
        curr_oper.push_str("fn main(){");
        let mut used = false;
        for oper in opers {
            if let Some(oper) = oper {
                match oper {
                    Macros::Output => {
                        curr_oper.push_str("print!");
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
                    Macros::String => {
                        curr_oper.push_str(&curr_string);
                    }
                    Macros::Quotes => {
                        curr_oper.push('"');
                    }
                }
            }
        }
        curr_oper.push(';');
        curr_oper.push('}');
        std::fs::write("run.rs", curr_oper).expect("Failed to write file");

        let output = std::process::Command::new("rustc")
            .args(&["run.rs"])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let run_output = std::process::Command::new("./run")
                .output()
                .expect("Failed to execute command");
            println!("{}", String::from_utf8_lossy(&run_output.stdout));
        } else {
            eprintln!("Compilation failed: {:?}", String::from_utf8_lossy(&output.stderr));
        }
        std::fs::remove_file("run.rs").expect("Failed to remove file");
    }
}
