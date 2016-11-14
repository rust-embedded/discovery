#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use pg::{delay, lsm303dlhc};

#[export_name = "main"]
#[inline(never)]
pub fn main() -> ! {
    loop {
        iprintln!("{:?}", lsm303dlhc::magnetic_field());
        delay::ms(1_000);
    }
}
