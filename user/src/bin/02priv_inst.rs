#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[app][02] Try to execute privileged instruction in U Mode");

    unsafe { asm!("sret") }
    0
}
