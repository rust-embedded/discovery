#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    iprintln!("Hello, world!");

    loop {}
}
