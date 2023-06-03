#![no_std]
#![no_main]
// PanicInfo::message 需要下面这个特性开关
#![feature(panic_info_message)]

use core::arch::global_asm;
#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use mm::frame_allocator::{frame_alloc, FrameTracker};

use crate::task::TASK_MANAGER;

mod config;
mod console;
mod lang_items;
mod loader;
mod mm;
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
    fn sbss_after_stack();
    fn ebss();

    fn stext();
    fn strampoline();
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
    // boot stack 在bss里面，所以
    clear_bss();

    debug!(".text\t[{:#x} ~ {:#x}]", stext as usize, etext as usize);
    debug!(
        ".rodata\t[{:#x} ~ {:#x}]",
        srodata as usize, erodata as usize
    );
    debug!(".data\t[{:#x} ~ {:#x}]", sdata as usize, edata as usize);
    debug!(".bss\t[{:#x} ~ {:#x}]", sbss as usize, ebss as usize);
    debug!(
        "boot st\t[{:#x} ~ {:#x}]",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    debug!(".ekernel\t{:#x}", ekernel as usize);

    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    mm::init();
    // heap_test();
    // frame_allocator_test();

    // TASK_MANAGER.show_debugging_info();

    info!("LAUNCHED!");
    TASK_MANAGER.run_first_app()
    // loop {}
}

fn clear_bss() {
    (sbss_after_stack as usize..ebss as usize)
        .for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

#[allow(unused)]
fn heap_test() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);

    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);

    let mut v = Vec::new();
    for i in 0..500 {
        v.push(i);
    }

    for i in 0..500 {
        assert_eq!(v[i], i);
    }

    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    println!("heap test passed!");
}

#[allow(unused)]
fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for _ in 0..5 {
        let f = frame_alloc().unwrap();
        println!("{:x?}", f);
        v.push(f)
    }
    v.clear();
    for _ in 0..5 {
        let f = frame_alloc().unwrap();
        println!("{:x?}", f);
        v.push(f)
    }
    drop(v);
    println!("frame_allocator test passed!");
}
