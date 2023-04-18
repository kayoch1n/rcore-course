use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let mut ret;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a7") id,
        )
    }
    ret
}

pub fn sys_exit(code: i32) -> isize {
    syscall(SYSCALL_EXIT, code as usize, 0, 0)
}

pub fn sys_write(fd: usize, buf:*const u8, len: usize) -> isize {
    syscall(SYSCALL_WRITE, fd, buf as usize, len)
}
