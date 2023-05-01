const SBI_CONSOLE_PUTCHAR: usize = 1;

use core::arch::asm;

// https://docs.rs/rustsbi/latest/rustsbi/#call-sbi-in-different-programming-languages
// eid: extension number to be placed in a7(x17)
// fid: function number to be placed in a6(x16)
// remaining arguments are placed from a0(x10) to a5(x15)
// a0(x10) is also used for returning code
#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a6") fid,
            in("a7") eid,
        );
    }
    ret
}

pub fn console_put_char(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, c, 0, 0);
}

pub fn shutdown() -> ! {
    sbi_call(
        sbi_spec::srst::EID_SRST,
        sbi_spec::srst::SYSTEM_RESET,
        0,
        0,
        0,
    );
    panic!("unreachable");
}
