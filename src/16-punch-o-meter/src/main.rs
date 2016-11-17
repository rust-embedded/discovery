#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use pg::I16x3;
use pg::{delay, lsm303dlhc};

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    loop {
        const FACTOR: f32 = 8. / ((1 << 15) as f32);

        let I16x3 { x, y, z } = lsm303dlhc::acceleration();

        let x = f32::from(x) * FACTOR;
        let y = f32::from(y) * FACTOR;
        let z = f32::from(z) * FACTOR;

        iprintln!("{:?}", (x, y, z));

        delay::ms(1_000);
    }
}
