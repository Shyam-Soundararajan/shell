use libc::{tcgetattr, tcsetattr, termios, ICANON, ECHO, TCSANOW};
use std::mem;
use std::io::{self, stdin, Read, Write, stdout};

//we could use crossterm and make this way easier but I hate myself so here we go
pub fn enable_raw_mode(){
    unsafe {
        //create a termios struct initialied with zeros and save the default config
        let mut term = mem::zeroed::<termios>();
        tcgetattr(0, &mut term);


        //there are three main termios flags
        //LFLAG - Local Flag : how the terminal itself handles flags
        //IFLAG - Input Flag : how incoming input is handled
        //OFLAG - Output Flag: how output is processed before being displayed


        //turn off echo and icanon
        term.c_lflag &= !ICANON;
        term.c_lflag &= !ECHO;
        tcsetattr(0, TCSANOW, &term);
    }
}

pub fn disable_raw_mode(){
    unsafe {
        let mut term = mem::zeroed::<termios>();
        tcgetattr(0, &mut term);
        term.c_lflag |= ICANON;
        term.c_lflag |= ECHO;
        tcsetattr(0, TCSANOW, &term);
    }
}

pub fn read_input(history: &[String]) -> String {
    let mut buffer = String::new();
    let mut cursor_pos = 0;
    
    let mut history_index = history.len();
    let mut stash = String::new();

    loop {
        //read 1 byte into a mutable buffer
        let mut b = [0];
        if stdin().read(&mut b).unwrap_or(0) == 0 { break; }
        let c = b[0] as char;


        // if new line or carriage return
        if c == '\n' || c == '\r'{
            print!("\r\n");
            stdout().flush().unwrap();
            break;
        }

        //backspace
        else if c == '\x7f' || c == '\x08' {
            if cursor_pos != 0 {
                buffer.remove(cursor_pos - 1);
                cursor_pos -= 1;
                print!("\x1b[D\x1b[K{}", &buffer[cursor_pos..]);
                if cursor_pos < buffer.len() {
                    print!("\x1b[{}D", buffer.len() - cursor_pos);
                }
                stdout().flush().unwrap();
            }
        }

        //arrow keys (escape sequence starts with \x1b)
        else if c == '\x1b' {
            let mut seq = [0; 2];
            if stdin().read(&mut seq).unwrap_or(0) == 2 && seq[0] == b'[' {
                if seq[1] == b'C' && cursor_pos < buffer.len() {
                    // Right Arrow
                    cursor_pos += 1;
                    print!("\x1b[C");
                    stdout().flush().unwrap();
                } 
                else if seq[1] == b'D' && cursor_pos > 0 {
                    // Left Arrow
                    cursor_pos -= 1;
                    print!("\x1b[D");
                    stdout().flush().unwrap();
                }
                else if seq[1] == b'A' {
                    // Up arrow
                    if history_index > 0 {
                        if history_index == history.len() {
                            stash = buffer.clone();
                        }
                        history_index -= 1;
                        buffer = history[history_index].clone();
                        
                        if cursor_pos > 0 {
                            print!("\x1b[{}D", cursor_pos); // Move left to start of prompt
                        }
                        cursor_pos = buffer.len();
                        print!("\x1b[K{}", buffer); // Clear line and print new buffer
                        stdout().flush().unwrap();
                    }
                }
                else if seq[1] == b'B' {
                    // Down arrow
                    if history_index < history.len() {
                        history_index += 1;
                        if history_index == history.len() {
                            buffer = stash.clone();
                        } else {
                            buffer = history[history_index].clone();
                        }
                        
                        if cursor_pos > 0 {
                            print!("\x1b[{}D", cursor_pos); // Move left to start of prompt
                        }
                        cursor_pos = buffer.len();
                        print!("\x1b[K{}", buffer); // Clear line and print new buffer
                        stdout().flush().unwrap();
                    }
                }
                else if seq[1] == b'3' {
                    // Delete
                    let mut next = [0; 1];
                    let _ = stdin().read(&mut next);
                    if cursor_pos < buffer.len() {
                        buffer.remove(cursor_pos);
                        print!("\x1b[K{}", &buffer[cursor_pos..]);
                        if cursor_pos < buffer.len() {
                            print!("\x1b[{}D", buffer.len() - cursor_pos);
                        }
                        stdout().flush().unwrap();
                    }
                }
                else if seq[1] == b'H' && cursor_pos > 0 {
                    // Home
                    print!("\x1b[{}D", cursor_pos);
                    cursor_pos = 0;
                    stdout().flush().unwrap();
                }
                else if seq[1] == b'F' && cursor_pos < buffer.len() {
                    // End
                    print!("\x1b[{}C", buffer.len() - cursor_pos);
                    cursor_pos = buffer.len();
                    stdout().flush().unwrap();
                }
            }
        }

        // for all other cases
        else if c.is_ascii() && !c.is_ascii_control() {
            buffer.insert(cursor_pos, c);
            cursor_pos += 1;
            print!("\x1b[K{}", &buffer[cursor_pos - 1..]);
            if cursor_pos < buffer.len() {
                print!("\x1b[{}D", buffer.len() - cursor_pos);
            }
            stdout().flush().unwrap();
        }


    }
    buffer
}