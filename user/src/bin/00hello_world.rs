#![no_std]
#![no_main]

// #[macro_use]
// extern crate user_lib;

use user_lib::println;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, world!");
    0
}
