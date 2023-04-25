#![no_std]
#![no_main]

use riscv::register::sstatus;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[app][03] reading csr");

    unsafe { sstatus::set_spp(sstatus::SPP::User) };
    0
}
