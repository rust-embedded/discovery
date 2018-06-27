#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate aux15;
#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;

use aux15::prelude::*;

entry!(main);

fn main() -> ! {
    let (_leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();

    loop {
        iprintln!(&mut itm.stim[0], "{:?}", lsm303dlhc.mag().unwrap());
        delay.delay_ms(1_000_u16);
    }
}
