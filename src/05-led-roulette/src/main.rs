#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate pg;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let y;
    let x = 42;
    y = x;

    loop {}
}
