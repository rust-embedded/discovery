#![deny(unsafe_code)]
#![no_std]

#[macro_use]
extern crate aux15;

use aux15::prelude::*;

fn main() {
    let (_leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();

    loop {
        iprintln!(&mut itm.stim[0], "{:?}", lsm303dlhc.mag().unwrap());
        delay.delay_ms(1_000_u16);
    }
}
