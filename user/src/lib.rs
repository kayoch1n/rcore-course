#![no_std]
#![no_main]
// 下面#[linkage = "weak"]需要这个特性开关
#![feature(linkage)]
// PanicInfo::message()需要这个特性开关
#![feature(panic_info_message)]

pub mod console;

mod lang_items;
mod syscall;

use buddy_system_allocator::LockedHeap;

const USER_HEAP_SIZE: usize = 16384;

static mut USER_HEAP: [u8; USER_HEAP_SIZE] = [0u8; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap::<64> = LockedHeap::empty();


extern "C" {
    fn sbss();
    fn ebss();
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    unsafe { HEAP.lock().init(USER_HEAP.as_ptr() as _, USER_HEAP_SIZE) }
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

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    syscall::sys_read(fd, buf)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    syscall::sys_write(fd, buf)
}

pub fn yield_() -> isize {
    syscall::sys_yield()
}

pub fn fork() -> isize {
    syscall::sys_fork()
}

pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match syscall::sys_waitpid(-1, exit_code as *mut i32 as _) {
            -2 => {
                yield_();
            }
            exit_code => return exit_code,
        }
    }
}

pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match syscall::sys_waitpid(pid as _, exit_code as *mut i32 as _) {
            -2 => {
                yield_();
            }
            exit_code => return exit_code,
        }
    }
}

pub fn exec(cmd: &str) -> isize {
    syscall::sys_exec(cmd)
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
