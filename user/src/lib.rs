#![no_std]
#![no_main]
// 下面#[linkage = "weak"]需要这个特性开关
#![feature(linkage)]
// PanicInfo::message()需要这个特性开关
#![feature(panic_info_message)]

pub mod console;

mod lang_items;
mod syscall;

extern "C" {
    fn sbss();
    fn ebss();
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main())
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("cannot find main")
}

fn clear_bss() {
    (sbss as usize..ebss as usize).for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) })
}

pub fn exit(code: i32) -> ! {
    syscall::sys_exit(code);
    panic!("exit")
}

pub fn write(fd: usize, buf: *const u8, len: usize) -> isize {
    syscall::sys_write(fd, buf, len)
}

pub fn yield_() -> isize {
    syscall::sys_yield()
}

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn get_time(val: &mut TimeVal) -> isize {
    syscall::sys_get_time(val)
}
