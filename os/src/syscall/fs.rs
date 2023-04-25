use core::{slice::from_raw_parts, str::from_utf8};

use crate::print;

const STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        STDOUT => {
            let slice = unsafe { from_raw_parts(buf, len) };
            let s: &str = from_utf8(slice).unwrap();
            print!("{}", s);
            len as isize
        }
        _ => {
            panic!("{fd} is not supported")
        }
    }
}
