#![allow(unused)]
mod builtins;
mod parser;
mod executor;
mod input;

//import write trait
use std::io::{self,Write};
fn main(){
    let mut history : Vec<String> = Vec::new();

    loop {
        input::enable_raw_mode();
        print!("[shyam {}]$ ", builtins::get_cur_dir());

        //force it onto the screen
        io::stdout().flush().unwrap();

        let input_str = input::read_input(&history);
        let input_str = input_str.trim();
        
        if !input_str.is_empty() {
            history.push(input_str.to_string());
        }

        let parsed_input = parser::parse_base(input_str);
        
        if parsed_input.is_empty(){
            continue;
        }

        let mut args: Vec<&str> = parsed_input.iter().map(|s| s.as_str()).collect();
        let mut background = false;
        if args.last() == Some(&"&") {
            background = true;
            args.pop();
            if args.is_empty() {
                continue;
            }
        }

        let is_builtin = builtins::execute(args[0], &args[1..]);
        if !is_builtin {
            input::disable_raw_mode();
            let _ = executor::execute(args[0], &args[1..], background);
        }
    }
}