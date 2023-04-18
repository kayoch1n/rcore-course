#![no_std]
// 这里居然也要 no_main
// 否则会报一个 requires `start` lang_item
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("hello, world!");
    0
}