#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate pg;

#[export_name = "main"]
#[inline(never)]
pub extern "C" fn main() -> ! {
    let y;
    let x = 42;
    y = x;

    loop {}
}
