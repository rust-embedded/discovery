# My solution

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

extern crate m;

use m::Float;
use pg::time::Instant;
use pg::{delay, lsm303dlhc};

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    use pg::time::FREQUENCY as SECONDS;

    const SENSITIVITY: f32 = 8. / ((1 << 15) as f32);
    const THRESHOLD: f32 = 0.5;
    const MEASUREMENT_TIME: u32 = 1 * SECONDS;

    let mut instant = None;
    let mut max_g = 0f32;
    loop {
        let g_x = (f32::from(lsm303dlhc::acceleration().x) * SENSITIVITY).abs();

        match instant {
            None => {
                // If acceleration goes above a threshold, we start measuring
                if g_x > THRESHOLD {
                    iprintln!("START!");

                    max_g = g_x;
                    instant = Some(Instant::now());
                }
            }
            // Still measuring
            Some(ref instant) if instant.elapsed() < MEASUREMENT_TIME => {
                if g_x > max_g {
                    max_g = g_x;
                }
            }
            _ => {
                // Report max value
                iprintln!("Max acceleration: {}g", max_g);

                // Measurement done
                instant = None;

                // Reset
                max_g = 0f32;
            }
        }

        delay::ms(50);
    }
}
```
