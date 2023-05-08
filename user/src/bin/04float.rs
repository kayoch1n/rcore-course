#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[inline(never)]
#[no_mangle]
fn foo() -> f32 {
    114.514
}

#[no_mangle]
fn main() -> i32 {
    println!("[app][04] floating-point arithmetic");
    let result = foo() * 2.0;
    println!("result={}", result);
    0
}