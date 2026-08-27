mod builtins;

//import write trait
use std::io::{self,Write};  

fn main(){
    loop {
        print!("[shyam]$ ");

        //force it onto the screen
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        let is_builtin = builtins::execute(input);



    }
}