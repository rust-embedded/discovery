#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let ms = 50_u8;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().ok();
            delay.delay_ms(ms);
            leds[curr].off().ok();
            delay.delay_ms(ms);
        }
    }
}
