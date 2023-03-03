#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

mod console;
mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));

extern "C" {
    fn sbss();
    fn ebss();

    fn stext();
    fn etext();

    fn srodata();
    fn erodata();

    fn sdata();
    fn edata();
}

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();

    debug!(".text\t[{:#x} ~ {:#x}]", stext as usize, etext as usize);
    debug!(".bss\t[{:#x} ~ {:#x}]", sbss as usize, ebss as usize);
    debug!(
        ".rodata\t[{:#x} ~ {:#x}]",
        srodata as usize, erodata as usize
    );
    debug!(".data\t[{:#x} ~ {:#x}]", sdata as usize, edata as usize);

    // panic!("Shutdown machine!")
    loop {}
}

fn clear_bss() {
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
