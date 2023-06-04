#![no_std]
#![no_main]


#[macro_use]
extern crate user_lib;

use user_lib::{TimeVal, get_time};


#[no_mangle]
fn main() -> i32 {

    let mut time_val = TimeVal { sec: 0, usec: 0};

    let err = get_time(&mut time_val);
    assert_eq!(err, 0);
    println!("time: {:?}", time_val);
    0
}