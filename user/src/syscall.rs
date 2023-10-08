use core::arch::asm;

use crate::TimeVal;

const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GETTIMEOFDAY: usize = 169;
const SYSCALL_CLONE: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAIT: usize = 260;

fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let mut ret;

    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") id,
        )
    }
    ret
}

pub fn sys_exit(code: i32) -> isize {
    syscall(SYSCALL_EXIT, code as usize, 0, 0)
}

pub fn sys_read(fd: usize, buf: &mut [u8]) -> isize {
    syscall(SYSCALL_READ, fd, buf.as_ptr() as _, buf.len())
}

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, fd, buf.as_ptr() as _, buf.len())
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, 0, 0, 0)
}

pub fn sys_get_time(val: &mut TimeVal) -> isize {
    syscall(SYSCALL_GETTIMEOFDAY, val as *mut TimeVal as usize, 0, 0)
}

pub fn sys_fork() -> isize {
    syscall(SYSCALL_CLONE, 0, 0, 0)
}

pub fn sys_waitpid(pid: isize, exit_code: *mut isize) -> isize {
    syscall(SYSCALL_WAIT, pid as usize, exit_code as usize, 0)
}

pub fn sys_exec(path: &str) -> isize {
    syscall(SYSCALL_EXEC, path.as_ptr() as usize, 0, 0)
}
