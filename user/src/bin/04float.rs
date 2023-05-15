#![no_std]
// 这里居然也要 no_main
// 否则会报一个 requires `start` lang_item
#![no_main]

#[macro_use]
extern crate user_lib;

#[inline(never)]
#[no_mangle]
fn foo(flag: bool) -> f64 {
    if flag {
        114.514
    } else {
        1919.810
    }
}

#[no_mangle]
fn main() -> i32 {
    let result = foo(true) * 2.0;
    println!("hello, world! - {}", result);
    0
}