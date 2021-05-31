# Take 2

This time, we'll use math to get the precise angle that the magnetic field forms with the X and Y
axes of the magnetometer.

We'll use the `atan2` function. This function returns an angle in the `-PI` to `PI` range. The
graphic below shows how this angle is measured:

<p align="center">
<img class="white_bg" title="atan2" src="https://upload.wikimedia.org/wikipedia/commons/0/03/Atan2_60.svg">
</p>

Although not explicitly shown in this graph the X axis points to the right and the Y axis points up.

Here's the starter code. `theta`, in radians, has already been computed. You need to pick which LED
to turn on based on the value of `theta`.

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

// You'll find this useful ;-)
use core::f32::consts::PI;

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, switch_hal::OutputSwitch, Direction, I16x3};
// this trait provides the `atan2` method
use m::Float;

#[entry]
fn main() -> ! {
    let (leds, mut lsm303dlhc, mut delay, _itm) = aux15::init();
    let mut leds = leds.into_array();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let _theta = (y as f32).atan2(x as f32); // in radians

        // FIXME pick a direction to point to based on `theta`
        let dir = Direction::Southeast;

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(100_u8);
    }
}
```

Suggestions/tips:

- A whole circle rotation equals 360 degrees.
- `PI` radians is equivalent to 180 degrees.
- If `theta` was zero, what LED would you turn on?
- If `theta` was, instead, very close to zero, what LED would you turn on?
- If `theta` kept increasing, at what value would you turn on a different LED?
