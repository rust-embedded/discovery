# Take 2

This time, we'll use math to get the precise angle that the magnetic field forms
with the X and Y axes of the magnetometer.

We'll use the `atan2` function. This function returns an angle in the `-PI` to
`PI` range. The graphic below shows how this angle is measured:

<p align="center">
<img title="atan2" src="https://upload.wikimedia.org/wikipedia/commons/0/03/Atan2_60.svg">
</p>

Here's the starter code. `theta` has already been computed. You need to pick
which LED to turn on based on the value of `theta`.

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate m;

#[macro_use]
extern crate pg;

// you'll find this useful ;-)
use core::f32::consts::PI;

// this trait provides the `atan2` method
use m::Float;
use pg::I16x3;
use pg::led::Direction;
use pg::{delay, led, lsm303dlhc};

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    loop {
        let I16x3 { x, y, .. } = lsm303dlhc::magnetic_field();

        let theta = (y as f32).atan2(x as f32);  // radians

        // TODO pick a direction to point to based on `theta`

        led::all_off();
        dir.on();

        delay::ms(100);
    }
}
```

Suggestions/tips:

- A whole circle rotation equals 360 degrees.
- `PI` radians is equivalent to 180 degrees.
- If `theta` was zero, what LED would you turn on?
- If `theta` was, instead, very close to zero, what LED would you turn on?
- If `theta` kept increasing, at what value would you turn on a different LED?
