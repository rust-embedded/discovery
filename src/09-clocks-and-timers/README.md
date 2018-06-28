# Clocks and timers

In this section, we'll re-implement the LED roulette application. I'm going to give you back the
`Led` abstraction but this time I'm going to take away the `Delay` abstraction `:-)`.

Here's the starter code. The `delay` function is unimplemented so if you run this program the LEDs
will blink so fast that they'll appear to always be on.

``` rust
#![no_main]
#![no_std]

extern crate aux9;
#[macro_use]
extern crate cortex_m_rt;

use aux9::tim6;

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // TODO implement this
}

entry!(main);

fn main() -> ! {
    let (mut leds, rcc, tim6) = aux9::init();

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
```
