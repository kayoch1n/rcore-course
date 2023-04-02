const SBI_CONSOLE_PUTCHAR: usize = 1;

const SRST_EXTENSION: usize = 0x53525354;
const SBI_SHUTDOWN: usize = 0;

use core::arch::asm;

#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x16") fid,
            in("x17") eid,
        );
    }
    ret
}

pub fn console_put_char(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, c, 0, 0);
}

pub fn shutdown() -> ! {
    sbi_call(SRST_EXTENSION, SBI_SHUTDOWN, 0, 0, 0);
    // FIXME: 这儿会递归爆栈，先记录下
    panic!("It should shutdown!");
}
