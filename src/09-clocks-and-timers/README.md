# Clocks and timers

In this section, we'll re-implement the LED roulette application. I'm going to give you back the
`Led` abstraction but this time I'm going to take away the `Delay` abstraction `:-)`.

Here's the starter code. The `delay` function is unimplemented so if you run this program the LEDs
will blink so fast that they'll appear to always be on.

``` rust
#![no_std]

extern crate aux;

use aux::tim6;

#[inline(never)]
fn delay(_tim6: &tim6::RegisterBlock, _ms: u16) {
    // TODO implement this
}

fn main() {
    let (mut leds, _rcc, tim6) = aux::init();

    // TODO initialize TIM7

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
```
