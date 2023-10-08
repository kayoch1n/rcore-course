#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use user_lib::{console::getchar, exec, fork, waitpid};

#[macro_use]
extern crate user_lib;

const LF: u8 = 0x0a;
const CR: u8 = 0x0d;
const DL: u8 = 0x7f;
const BS: u8 = 0x08;

fn main() -> i32 {
    println!("Rust user shell!");
    let mut line = String::new();
    print!(">> ");
    loop {
        match getchar() {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    line.push('\0');
                    let pid = fork();
                    if pid == 0 {
                        // child process
                        if exec(line.as_str()) == -1 {
                            println!("error when executing");
                            return -4;
                        }
                        unreachable!()
                    } else {
                        // parent process
                        let mut exit_code: i32 = 0;
                        let exit_pid = waitpid(pid as usize, &mut exit_code);
                        assert_eq!(pid, exit_pid);
                        println!("shell: proc {} exited with {}", pid, exit_code)
                    }
                    line.clear()
                }
                print!(">> ")
            }
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            ch => {
                print!("{}", ch as char);
                line.push(ch as char)
            }
        }
    }
}
