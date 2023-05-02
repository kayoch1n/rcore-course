#![no_std]
// 这里居然也要 no_main
// 否则会报一个 requires `start` lang_item
#![no_main]

use user_lib::yield_;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    yield_();
    println!("[app][00] hello, world!");
    0
}
