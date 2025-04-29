#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 500_u16;

    loop {
        leds[0].on().ok();
        delay.delay_ms(half_period);

        leds[0].off().ok();
        delay.delay_ms(half_period);
    }
}
