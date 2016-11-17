#![no_std]
#![no_main]

extern crate pg;

use core::iter;

use pg::led::LEDS;
use pg::peripheral;

#[inline(never)]
fn delay(ms: u16) {
    // TODO implement this
}

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let (rcc, tim7) =
        unsafe { (peripheral::rcc_mut(), peripheral::tim7_mut()) };

    // TODO initialize TIM7

    loop {
        for (current, next) in LEDS.iter()
            .zip(LEDS.iter().skip(1).chain(iter::once(&LEDS[0]))) {
            next.on();
            delay(50);
            current.off();
            delay(50);
        }
    }
}
