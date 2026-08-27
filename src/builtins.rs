//exit
use std::process;
//filesystem manipulation
use std::fs;
//curdir
use std::env;
fn  ls(cur_dir: &str){
    let entries = fs::read_dir(cur_dir).unwrap();
    for entry in entries{
        let entry = entry.unwrap();
        print!("{} ",entry.file_name().to_string_lossy());
    }
    println!();
}
pub fn get_cur_dir() -> String{
    env::current_dir().unwrap().to_str().unwrap().to_string()
}
pub fn execute(cmd: &str, args: &[&str]) -> bool{
    match cmd{
        "exit" => {
            process::exit(0);
        }

        "ls" => {
            ls(".");
            true
        }

        "cd" => {
            if args.len() == 1{
                match env::set_current_dir(args[0]){
                    Ok(_) => {}
                    Err(_) => {
                        println!("Cannot find directory: {}",args[0]);
                    }
                }
            }
            else{
                println!("Too many arguments");
            }
            true
        }

        "clear" => {
            println!("\x1B[2J\x1B[H");
            true
        }

        _ => {
            false
        }
    }
}