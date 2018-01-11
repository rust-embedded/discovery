#![no_std]

extern crate aux;

use aux::tim6;

#[inline(never)]
fn delay(_tim6: &tim6::RegisterBlock, _ms: u16) {
    // TODO implement this
}

fn main() {
    let (mut leds, _rcc, tim6) = aux::init();

    // TODO initialize TIM6

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay(tim6, ms);
            leds[curr].off();
            delay(tim6, ms);
        }
    }
}
