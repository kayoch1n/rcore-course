#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const LEN: usize = 100;

#[no_mangle]
fn main() -> i32 {
    let p = 7u64;
    let m = 998244353u64;
    let iter = 400000usize;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;

    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            println!("{} [{}/{}]", file!(), i, iter);
        }
    }

    println!("{}^{}={}(mod {})", p, iter, s[cur], m);
    println!("{} done", file!());
    0
}