use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;

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

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    syscall(SYSCALL_WRITE, fd, buf as usize, len)
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, 0, 0, 0)
}
