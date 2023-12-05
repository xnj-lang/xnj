pub mod global_state {
    pub struct GlobalState {
        pub strings_contents: Vec<String>,
        pub variable_names: Vec<String>,
        pub variable_value: Vec<i32>,
        pub increment_names: Vec<String>,
        pub fmt_string: Vec<String>,
        pub decrement_names: Vec<String>,

    }

    impl Default for GlobalState {
        fn default() -> Self {
            GlobalState {
                strings_contents: Vec::new(),
                variable_names: Vec::new(),
                variable_value: Vec::new(),
                increment_names: Vec::new(),
                fmt_string: Vec::new(),
                decrement_names: Vec::new(),
            }
        }
    }
}

pub mod parser_state {
    pub struct ParserState {
        pub current_operation: String,
        pub fmt_variable_value: i32,
        pub fmt_name_variable: String,
        pub fmt_increment_variable_name: String,
        pub fmt_decrement_variable_name: String,
        pub bracket_is_used: bool,
        pub linefeed: bool,
        pub increment_index: usize,
        pub decrement_index: usize,
        pub number_index: usize,
        pub variable_name_index: usize,
        pub current_index: usize,
        pub string_index: usize,
        pub curly_index: usize
    }

    impl Default for ParserState {
        fn default() -> Self {
            ParserState {
                current_operation: String::new(),
                fmt_variable_value: 0,
                fmt_name_variable: String::new(),
                fmt_increment_variable_name: String::new(),
                fmt_decrement_variable_name: String::new(),
                bracket_is_used: false,
                linefeed: false,
                increment_index: 0,
                decrement_index: 0,
                number_index: 0,
                variable_name_index: 0,
                current_index: 0,
                string_index: 0,
                curly_index: 0
            }
        }
    }
}

pub mod lexer_state {
    use crate::syntax_analyzer::main::tokens;
    pub struct LexerState {
        pub operations: Vec<Option<tokens::Token>>,
        pub current_fmt_variable_name: String,
        pub variable_name: String,
        pub current_command: String,
        pub current_string_content: String,
        pub current_number: String,
        pub count_quotes: u32,
        pub count_curly: u32,
        pub string_operation: bool,
        pub is_variable_name: bool,
        pub is_number: bool,
        pub fmt_operation: bool,
    }

    impl Default for LexerState {
        fn default() -> Self {
            LexerState {
                operations: Vec::new(),
                current_fmt_variable_name: String::new(),
                variable_name: String::new(),
                current_command: String::new(),
                current_string_content: String::new(),
                current_number: String::new(),
                count_quotes: 0,
                count_curly: 0,
                string_operation: false,
                is_variable_name: false,
                is_number: false,
                fmt_operation: false,
            }
        }
    }
}