#![allow(unused)]
mod builtins;
mod parser;
mod executor;

//import write trait
use std::io::{self,Write};

fn main(){
    loop {
        print!("[shyam {}]$ ", builtins::get_cur_dir());

        //force it onto the screen
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        let input = parser::parse_base(input);
        let is_builtin = builtins::execute(input[0],&input[1..]);
        if !(is_builtin){
            executor::execute(input[0],&input[1..]);
            //check if external


        }


    }
}