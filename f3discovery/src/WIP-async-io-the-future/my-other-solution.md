# My other solution

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate fixedvec;

#[macro_use]
extern crate pg;

use core::ops::Not;

use fixedvec::FixedVec;
use pg::{Async, Future, Serial, Timer};
use pg::led::LEDS;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let mut timer = Timer::new().unwrap();
    let Serial { mut rx, .. } = Serial::new().unwrap();

    let mut periodic = timer.periodic(100);
    let mut bytes = rx.bytes();

    let mut memory = alloc_stack!([u8; 32]);
    let mut buffer = FixedVec::new(&mut memory);
    let mut direction = Direction::Clockwise;
    let mut state = 0;
    loop {
        if let Async::Ready(()) = periodic.poll() {
            match direction {
                Direction::Clockwise => {
                    if state == 7 {
                        LEDS[state].off();
                        LEDS[0].on();

                        state = 0;
                    } else {
                        LEDS[state].off();
                        LEDS[state+1].on();

                        state += 1;
                    }
                }
                Direction::Counterclockwise => {
                    if state == 0 {
                        LEDS[state].off();
                        LEDS[7].on();

                        state = 7;
                    } else {
                        LEDS[state].off();
                        LEDS[state-1].on();

                        state -= 1;
                    }
                }
            }
        }

        if let Async::Ready(byte) = bytes.poll() {
            if let Err(_) = buffer.push(byte) {
                // TODO report error
                buffer.clear();
            } else if byte == '\r' as u8 {
                if buffer.as_slice() == b"reverse\r" {
                    direction = !direction;
                }

                buffer.clear();
            }
        }
    }
}

enum Direction {
    Clockwise,
    Counterclockwise,
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Direction::Clockwise => Direction::Counterclockwise,
            Direction::Counterclockwise => Direction::Clockwise,
        }
    }
}
```
