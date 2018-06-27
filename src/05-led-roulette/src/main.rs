#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate aux5;
#[macro_use]
extern crate cortex_m_rt;

entry!(main);

fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
