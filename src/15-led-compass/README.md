# LED compass

In this section, we'll implement a compass using the LEDs on the F3. Like proper compasses, our LED
compass must point north somehow. It will do that by turning on one of its eight LEDs; the on LED
should point towards north.

Magnetic fields have both a magnitude, measured in Gauss or Teslas, and a *direction*. The
magnetometer on the F3 measures both the magnitude and the direction of an external magnetic field
but it reports back the *decomposition* of said field along *its axes*.

See below, the magnetometer has three axes associated to it.

<p align="center">
<img height=480 title="Magnetometer axes" src="assets/f3-lsm303dlhc.png">
</p>

Only the X and Y axes are shown above. The Z axis is pointing "out" of your screen.

Let's get familiar with the readings of the magnetometer by running the following starter code:

``` rust
#![deny(unsafe_code)]
#![no_std]

#[macro_use]
extern crate aux15;

use aux15::prelude::*;

fn main() {
    let (_leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();

    loop {
        iprintln!(&mut itm.stim[0], "{:?}", lsm303dlhc.mag().unwrap());
        delay.delay_ms(1_000_u16);
    }
}
```

This `lsm303dlhc` module provides high level API over the LSM303DLHC. Under the hood it does the
same I2C routine that you implemented in the last section but it reports the X, Y and Z values in a
`I16x3` struct instead of a tuple.

Locate where north is at your current location. Then rotate the board such that it's aligned
"towards north": the North LED (LD3) should be pointing towards north.

Now run the starter code and observe the output. What X, Y and Z values do you see?

``` console
$ # itmdump terminal
(..)
I16x3 { x: -106, y: -1, z: -553 }
I16x3 { x: -101, y: 0, z: -555 }
I16x3 { x: -108, y: 1, z: -559 }
```

Now rotate the board 90 degrees while keeping it parallel to the ground. What X, Y and Z values do
you see this time? Then rotate it 90 degrees again. What values do you see?
