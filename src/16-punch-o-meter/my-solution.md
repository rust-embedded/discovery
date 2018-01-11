# My solution

``` rust
#![deny(unsafe_code)]
#![no_std]

#[macro_use]
extern crate aux;
extern crate m;

use aux::Sensitivity;
use aux::prelude::*;
use m::Float;

fn main() {
    const SENSITIVITY: f32 = 12. / (1 << 14) as f32;
    const THRESHOLD: f32 = 0.5;

    let (mut lsm303dlhc, mut delay, mono_timer, mut itm) = aux::init();

    lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();

    let measurement_time = mono_timer.frequency().0; // 1 second in ticks
    let mut instant = None;
    let mut max_g = 0.;
    loop {
        let g_x = f32::from(lsm303dlhc.accel().unwrap().x).abs() * SENSITIVITY;

        match instant {
            None => {
                // If acceleration goes above a threshold, we start measuring
                if g_x > THRESHOLD {
                    iprintln!(&mut itm.stim[0], "START!");

                    max_g = g_x;
                    instant = Some(mono_timer.now());
                }
            }
            // Still measuring
            Some(ref instant) if instant.elapsed() < measurement_time => {
                if g_x > max_g {
                    max_g = g_x;
                }
            }
            _ => {
                // Report max value
                iprintln!(&mut itm.stim[0], "Max acceleration: {}g", max_g);

                // Measurement done
                instant = None;

                // Reset
                max_g = 0.;
            }
        }

        delay.delay_ms(50_u8);
    }
}
```
