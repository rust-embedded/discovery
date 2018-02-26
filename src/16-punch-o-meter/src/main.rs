#![deny(unsafe_code)]
#![no_std]

#[macro_use]
extern crate aux16;
extern crate m;

use aux16::{I16x3, Sensitivity};
use aux16::prelude::*;

fn main() {
    let (mut lsm303dlhc, mut delay, _mono_timer, mut itm) = aux16::init();

    // extend sensing range to `[-12g, +12g]`
    lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();
    loop {
        const SENSITIVITY: f32 = 12. / (1 << 14) as f32;

        let I16x3 { x, y, z } = lsm303dlhc.accel().unwrap();

        let x = f32::from(x) * SENSITIVITY;
        let y = f32::from(y) * SENSITIVITY;
        let z = f32::from(z) * SENSITIVITY;

        iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));

        delay.delay_ms(1_000_u16);
    }
}
