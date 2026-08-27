use std::process;
use std::fs;
pub fn execute(input: &str) -> bool{
    match input{
        "exit" => {
            process::exit(0);
        }
        "ls" => {
            
        }
        _ => {
            return false;
        }
    }
}