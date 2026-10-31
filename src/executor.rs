
//unix os funcs
use libc::{fork, execve, waitpid};
//null ptr
use std::ptr;
//convert rust string to C
use std::ffi::CString;
use std::os::raw::c_char;
//env vars
use std::env;
use crate::parser::parse_path;
//path validity
use std::path::Path;

pub fn execute(cmd: &str, args: &[&str], background: bool) -> Result<(), String> {
    let mut cmd_path = cmd.to_string();
    let pid = unsafe { fork() };

    if pid < 0 {
        Err("fork failed".to_string())
    }

    else if pid == 0 {
        //path resolution & handling
        let is_path = parse_path(cmd);
        if is_path {
            if Path::new(cmd).exists() {
                // check if dir
                if Path::new(cmd).is_dir() {
                    println!("{} is a directory!", cmd);
                    std::process::exit(1);
                }
            }
            else{
                println!("{}: No such file or directory!", cmd);
                std::process::exit(1);
            }
        }
        //search $PATH for the entered command
        else{
            let path_list = env::var_os("PATH");
            let mut valid_ext = false;
            if let Some(paths) = path_list {
                for i in paths.to_string_lossy().split(':') {
                    let full_path = i.to_string() + "/" + cmd;
                    if Path::new(&full_path).exists(){
                        cmd_path = full_path;
                        valid_ext = true;
                        break;
                    }
                }
            }
            if !valid_ext {
                println!("{}: command not found", cmd);
                std::process::exit(1);
            }
        }

        //convert cmd to C string
        let c_cmd = CString::new(cmd_path).unwrap();

        //convert args to C string
        let mut c_args = Vec::new();
        c_args.push(c_cmd);
        for arg in args {
            c_args.push(CString::new(*arg).unwrap());
        }
        //create an array of ptrs to C string
        let mut argv: Vec<*const c_char> = Vec::new();
        for arg in &c_args {
            argv.push(arg.as_ptr());
        }
        argv.push(ptr::null());

        //env vars
        let mut c_envs: Vec<CString> = Vec::new();
        for (key, value) in env::vars() {
            c_envs.push(
                CString::new(format!("{}={}", key, value)).unwrap()
            );
        }
        //envp pointer array
        let mut envp: Vec<*const c_char> = Vec::new();
        for env in &c_envs {
            envp.push(env.as_ptr());
        }
        envp.push(ptr::null());

        unsafe {
            let execve_status = execve(c_args[0].as_ptr(), argv.as_ptr(), envp.as_ptr());
            if execve_status != 0 {
                let err = std::io::Error::last_os_error();
                println!("{}: {}", cmd, err);
                std::process::exit(1);
            }
        }
        std::process::exit(0);
    }

    else {
        if background {
            println!("[{}]", pid);
            return Ok(());
        }

        let result = unsafe {
            waitpid(pid, ptr::null_mut(), 0)
        };

        if result < 0 {
            return Err("waitpid failed".to_string());
        }

        Ok(())
    }
}