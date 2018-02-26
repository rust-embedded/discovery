#![deny(unsafe_code)]
#![no_std]

extern crate aux5;

fn main() {
    let _y;
    let x = 42;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
