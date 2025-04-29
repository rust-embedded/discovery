#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use core::iter;

use pg::led::LEDS;
use pg::{Async, Future, Timer};

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let mut timer = Timer::new().unwrap();

    let mut periodic = timer.periodic(100);
    let mut leds = LEDS.iter()
        .zip(LEDS.iter().skip(1))
        .chain(iter::once((&LEDS[7], &LEDS[0])))
        .cycle();
    loop {
        if let Async::Ready(()) = periodic.poll() {
        if let Some((current, next)) = leds.next() {
            current.off();
            next.on();
        }
    }
    }
}
