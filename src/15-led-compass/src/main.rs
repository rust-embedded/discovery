#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use pg::{delay, lsm303dlhc};

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    loop {
        iprintln!("{:?}", lsm303dlhc::magnetic_field());
        delay::ms(1_000);
    }
}
