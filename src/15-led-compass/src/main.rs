#![deny(unsafe_code)]
#![no_std]

#[macro_use]
extern crate aux;

use aux::prelude::*;

fn main() {
    let (_leds, mut lsm303dlhc, mut delay, mut itm) = aux::init();

    loop {
        iprintln!(&mut itm.stim[0], "{:?}", lsm303dlhc.mag().unwrap());
        delay.delay_ms(1_000_u16);
    }
}
