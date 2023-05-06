#![no_std]
#![no_main]
// PanicInfo::message 需要下面这个特性开关
#![feature(panic_info_message)]

use core::arch::global_asm;

use crate::task::TASK_MANAGER;

mod config;
mod console;
mod lang_items;
mod loader;
mod sbi;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.asm"));

extern "C" {
    fn sbss();
    fn ebss();

    fn stext();
    fn etext();

    fn srodata();
    fn erodata();

    fn sdata();
    fn edata();

    fn boot_stack_lower_bound();
    fn boot_stack_top();

    fn ekernel();
}

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();

    debug!(".text\t[{:#x} ~ {:#x}]", stext as usize, etext as usize);
    debug!(
        ".rodata\t[{:#x} ~ {:#x}]",
        srodata as usize, erodata as usize
    );
    debug!(".data\t[{:#x} ~ {:#x}]", sdata as usize, edata as usize);
    debug!(
        "boot st\t[{:#x} ~ {:#x}]",
        boot_stack_lower_bound as usize, boot_stack_top as usize
    );
    debug!(".bss\t[{:#x} ~ {:#x}]", sbss as usize, ebss as usize);
    debug!(".ekernel\t{:#x}", ekernel as usize);

    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    loader::init();

    TASK_MANAGER.show_debugging_info();

    info!("LAUNCHED!");
    TASK_MANAGER.run_first_app()
    // loop {}
}

fn clear_bss() {
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
