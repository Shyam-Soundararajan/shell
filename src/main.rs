#![allow(unused)]
mod builtins;
mod parser;
mod executor;
mod input;

//import write trait
use std::io::{self,Write};

fn main(){


    loop {
        input::enable_raw_mode();
        print!("[shyam {}]$ ", builtins::get_cur_dir());

        //force it onto the screen
        io::stdout().flush().unwrap();

        let input_str = input::read_input();
        let input_str = input_str.trim();
        let parsed_input = parser::parse_base(input_str);
        if parsed_input.is_empty(){
            continue;
        }
        let is_builtin = builtins::execute(parsed_input[0],&parsed_input[1..]);
        if !(is_builtin){
            input::disable_raw_mode();
            let _ = executor::execute(parsed_input[0],&parsed_input[1..]);
        }
    }
}