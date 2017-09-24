# Clocks and timers

Summary:

- Same routine: power up, configure
- As before, I'll point you to the documentation
- APB1 Clock: 8 MHz
- Configuration: one shot, autoreload, prescaler
- Introduce: busy waiting `while !tim7.sr.read().uif() {}`
- Re-implement `delay::ms`

---

In this section, we'll re-implement the LED roulette application. I'm going to
give you back the `led` module but this time I'm going to take away the `delay`
module `:-)`.

Here's the starter code. The `delay` function is unimplemented so if you run
this program the LEDs will blink so fast that they'll appear to always be on.

``` rust
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
    unsafe {
        let rcc = peripheral::rcc_mut();
        let tim7 = peripheral::tim7_mut();
    }

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
```

---

Functional description:

- The timer is enabled
- A counter register increments its value on each "tick"
- When the counter reaches the value held in the auto-reload register, it will
  reset back to zero and generate an *update* event.

The timer can operate in two modes:

- Continuous mode: After an update event, the timer will start counting again.
- One pulse mode: After an update event, the timer will stop.

How long does this "tick" last? That's determined by the APB1 clock and the counter and prescaler settings.

TIM6/TIM7 registers - Section 22.4 - Page 681 - Reference Manual.

Initialization is as usual: power up the peripheral then configure it.

- Use `TIM7EN` in `RCC::APB1ENR` to power up the peripheral.
- `TIM7::CR1`, the configuration register.
- `TIM7::PSC`, the prescaler register.
- `TIM7::ARR`, the auto-reload register.
- `TIM7::CNT`, the counter register.
- `TIM7::SR`, the status register, indicates if an update event has occurred.
- `TIM7::EGR`, the event generation register. Can be used to generate an update.
  event.

The catch: the auto-reload register is buffered. When you write to it, it won't
immediately change until the *next* update event. You can synthesize an update
event using EGR.
