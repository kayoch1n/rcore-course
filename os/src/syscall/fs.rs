use core::str::from_utf8;

use crate::{mm::translate_byte_buffer, print, task::TASK_MANAGER};

const STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        STDOUT => {
            let token = TASK_MANAGER.get_current_token();
            for slice in translate_byte_buffer(token, buf, len) {
                let s: &str = from_utf8(slice).unwrap();
                print!("{}", s);
            }
            len as isize
        }
        _ => {
            panic!("{fd} is not supported")
        }
    }
}
