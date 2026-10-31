//exit
use std::process;
//filesystem manipulation
use std::fs;
//curdir
use std::env;

fn ls(cur_dir: &str){
    match fs::read_dir(cur_dir) {
        Ok(entries) => {
            for entry in entries{
                if let Ok(entry) = entry {
                    print!("{} ",entry.file_name().to_string_lossy());
                }
            }
            println!();
        }
        Err(e) => {
            println!("ls: cannot access '{}': {}", cur_dir, e);
        }
    }
}

pub fn get_cur_dir() -> String{
    env::current_dir().unwrap().to_str().unwrap().to_string()
}

pub fn execute(cmd: &str, args: &[&str]) -> bool{
    match cmd{
        "exit" => {
            crate::input::disable_raw_mode();
            process::exit(0);
        }

        "ls" => {
            if args.len() == 0 {
                ls(".");
            } else {
                for arg in args {
                    if args.len() > 1 {
                        println!("{}:", arg);
                    }
                    ls(arg);
                }
            }
            true
        }

        "cd" => {
            if args.len() == 1{
                match env::set_current_dir(args[0]){
                    Ok(_) => {}
                    Err(_) => {
                        println!("cd: {}: No such file or directory",args[0]);
                    }
                }
            }
            else if args.len() == 0{
                if let Ok(home) = env::var("HOME") {
                    if let Err(_) = env::set_current_dir(&home) {
                        println!("cd: {}: No such file or directory", home);
                    }
                } else {
                    println!("cd: HOME not set");
                }
            }
            else{
                println!("cd: too many arguments");
            }
            true
        }
        "pwd" => {
            println!("{}",get_cur_dir());
            true
        }
        "clear" => {
            println!("\x1B[2J\x1B[H");
            true
        }
        "echo" => {
            println!("{}", args.join(" "));
            true
        }

        _ => {
            false
        }
    }
}