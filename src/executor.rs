//unix os funcs
use libc::{fork, execve, waitpid};
//null ptr
use std::ptr;
//convert rust string to C
use std::ffi::CString;
use std::os::raw::c_char;
//env vars
use std::env;

pub fn execute(cmd: &str, args: &[&str]) -> Result<(), String> {
    let pid = unsafe { fork() };

    if pid < 0 {
        return Err("fork failed".to_string());
    }

    if pid == 0 {
        //convert cmd to C string
        let c_cmd = CString::new(cmd).unwrap();

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
            execve(c_args[0].as_ptr(), argv.as_ptr(), envp.as_ptr());
        }

        //execve only returns if it failed
        return Err("execve failed".to_string());
    }

    if pid > 0 {
        let result = unsafe {
            waitpid(pid, ptr::null_mut(), 0)
        };

        if result < 0 {
            return Err("waitpid failed".to_string());
        }

        Ok(())
    } else {
        unreachable!();
    }
}