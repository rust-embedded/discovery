# Putting it all together

``` rust
#![no_std]
#![no_main]

extern crate pg;

use core::iter;

use pg::led::LEDS;
use pg::peripheral;

#[inline(never)]
fn delay(ms: u16) {
    let tim7 = unsafe { peripheral::tim7_mut() };

    // Set the timer to go off in `ms` ticks
    // 1 tick = 1 ms
    tim7.arr.write(|w| w.arr(ms));

    // CEN: Enable the counter
    tim7.cr1.modify(|_, w| w.cen(true));

    // Wait until the alarm goes off (the "update event" occurs)
    while !tim7.sr.read().uif() {}

    // Clear the "update" flag
    tim7.sr.write(|w| w);
}

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let (rcc, tim7) =
        unsafe { (peripheral::rcc_mut(), peripheral::tim7_mut()) };

    // Power on the TIM7 timer
    rcc.apb1enr.modify(|_, w| w.tim7en(true));

    // OPM Select one pulse mode
    // CEN Keep the counter disabled for now
    tim7.cr1.write(|w| w.opm(false).cen(false));

    // Configure the prescaler to have the counter operate at 1 KHz
    // APB1_CLOCK = 8 MHz
    // PSC = 7999
    // 8 MHz / (7999 + 1) = 1 KHz
    // The counter (CNT) will increase on every millisecond
    tim7.psc.write(|w| w.psc(7_999));

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
``
