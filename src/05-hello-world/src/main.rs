#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

#[export_name = "main"]
#[inline(never)]
pub extern "C" fn main() -> ! {
    iprintln!("Hello, world!");

    loop {}
}
