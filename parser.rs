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
        Linefeed,
        INT32,
        Curly,
        Fmt,
        PP
    }

    pub fn parse(code: &str) {
        if metadata("run.rs").is_ok() { remove_file("run.rs").expect("Failed to remove file");}

        let macros_vec: Vec<&str> = vec!["op", "(", ")", "\"", ";", "main[Status]", "Ok()", "}", "opln", "crt-int-:", "{"];
        let keyw: Vec<&str> = vec!["++"];
        let code = code.chars().collect::<Vec<char>>();
        let mut opers: Vec<Option<Macros>> = Vec::new();
        let mut strings = Vec::new();
        let mut varnames = Vec::new();
        let mut varval: Vec<i32> = Vec::new();
        let mut fmts: Vec<String> = Vec::new();
        let mut ppnames: Vec<String> = Vec::new();

        let mut command = String::new();
        let mut curr_string = String::new();
        let mut curr_oper = String::new();
        let mut curr_fmt = String::new();
        let mut varname = String::new();
        let mut number = String::new();
        let mut fmtn = "".to_string();
        let mut fmtp = "".to_string();

        let mut count_quotes = 0;
        let mut count_curly = 0;
        let mut fmtv = 0;

        let mut string_oper = false;
        let mut used = false;
        let mut lf = false;
        let mut name = false;
        let mut itsnumber = false;
        let mut fmtoper = false;

        let mut ppindex = 0;
        let mut intindex = 0;
        let mut nameindex = 0;
        let mut current_index = 0;
        let mut stringindex = 0;
        let mut curlyindex = 0;

        for i in 0..code.len() {
            command.push(code[i]);
            if code[i].is_whitespace() && !string_oper {
                if !command.trim().is_empty() { continue; }
                else {
                    command.clear();
                    continue;
                }
            }
            else if name{
                if code[i] != ':'{ varname.push(code[i]); }
                else {
                    varnames.push(varname.clone());
                    varname.clear();
                    name = false;
                    itsnumber = true;
                }
            }
            else if itsnumber{
                if code[i] != ';'{
                    number.push(code[i]);
                }
                else{
                    number.parse::<i32>().expect("Error");
                    opers.push(Some(Macros::Semicolon));
                    varval.push(number.parse().unwrap());
                    number.clear();
                    itsnumber = false;
                    command.clear();
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
            else if fmtoper && code[i] != '}' {
                if let Some(&Some(Macros::Fmt)) = opers.last() {
                    curr_fmt.push(code[i]);
                    command.clear();
                } else {
                    opers.push(Some(Macros::Fmt));
                    curr_string.push(code[i]);
                    command.clear();
                }
            }
            else if keyw.iter().any(|&s| command.contains(s)) {
                if command.ends_with("++") {
                    if let Some(index) = command.rfind("++") { ppnames.push(command[..index].to_string()); }
                    opers.push(Some(Macros::PP));
                    command.clear();
                }
            }
            else if macros_vec.iter().any(|&s| s.contains(&*command)) {
                match command.as_str() {
                    "crt-int-" =>{
                        opers.push(Some(Macros::INT32));
                        command.clear();
                        name = true;
                    }
                    "op" => {
                        if code[i+1] != 'l' {
                            opers.push(Some(Macros::Output));
                            command.clear();
                        }
                        else{ continue }
                    }
                    "{" => {
                        opers.push(Some(Macros::Curly));
                        command.clear();
                        count_curly += 1;
                        if count_curly == 1{
                            fmtoper = true;
                        }
                        else{
                            fmts.push(curr_fmt.clone());
                            curr_fmt.clear();
                            fmtoper = false;
                            count_curly = 0;
                        }
                    }
                    "\"" => {
                        opers.push(Some(Macros::Quotes));
                        command.clear();
                        count_quotes += 1;
                        if count_quotes == 1 {
                            string_oper = true;
                        } else {
                            strings.push(curr_string.clone());
                            curr_string.clear();
                            string_oper = false;
                            count_quotes = 0;
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
                if varnames.len() - 1 >= nameindex { fmtn = varnames[nameindex].clone(); }
                if varval.len() -1 >= intindex { fmtv = varval[intindex]; }
                if ppnames.len() -1 >= ppindex { fmtp = ppnames[ppindex].clone(); }
                match oper {
                    Macros::INT32 => {
                        curr_oper.push_str(&*format!("let mut {fmtn}: i32 = {fmtv}"));
                        nameindex+=1;
                        intindex+=1;
                    }
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
                    Macros::PP => {
                        curr_oper.push_str(&*format!("{fmtp}+=1;"));
                        ppindex+=1;
                    }
                    Macros::Fmt => {
                        curr_oper.push_str(&fmts[curlyindex]);
                        curlyindex+=1;
                    }
                    Macros::String => {
                        curr_oper.push_str(&strings[stringindex]);
                        stringindex+=1;
                    }
                    Macros::Curly => curr_oper.push('{'),
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
            remove_file("run.rs").expect("Failed to remove file");
            exit(1);
        } else {
            remove_file("run.rs").expect("Failed to remove file");
            eprintln!("Process finished with exit status Err");
            exit(1);
        }
    }
}
