# Magnitude

We have been working with the direction of the magnetic field but what's its
real magnitude? The number that the `magnetic_field` function reports are
unit-less. How can we convert those values to Gauss?

The documentation will answer that question.

> Section 2.1 Sensor characteristics - Page 10 - LSM303DLHC Data Sheet

The table in that page shows a "magnetic gain setting" that has different values
according to the values of the GN bits. By default, those GN bits are set to
`001`. That means that magnetic gain of the X and Y axes is `1100 LSB / Gauss`
and the magnetic gain of the Z axis is `980 LSB / Gauss`. LSB stands for Least
Significant Bits and the `1100 LSB / Gauss` number indicates that a reading of
`1100` is equivalent to `1 Gauss`, a reading of `2200` is equivalent to 2 Gauss
and so on.

So, what we need to do is divide the X, Y and Z values that the sensor outputs
by its corresponding "gain". Then, we'll have the X, Y and Z components of the
magnetic field in Gauss.

With some extra math we can retrieve the magnitude of the magnetic field from
its X, Y and Z components:

``` rust
let magnitude = (x * x + y * y + z * z).sqrt();
```

Putting all this together in a program:

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate m;

#[macro_use]
extern crate pg;

use m::Float;
use pg::I16x3;
use pg::lsm303dlhc;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    const XY_GAIN: f32 = 1100.; // LSB / G
    const Z_GAIN: f32 = 980.; // LSB / G

    loop {
        let I16x3 { x, y, z } = lsm303dlhc::magnetic_field();
        let x = f32::from(x) / XY_GAIN;
        let y = f32::from(y) / XY_GAIN;
        let z = f32::from(z) / Z_GAIN;

        let mag = (x * x + y * y + z * z).sqrt();

        iprintln!("{} mG", mag * 1_000.);
    }
}
```

This program will report the magnitude of the magnetic field in milligauss
(`mG`) because the magnitude of the Earth's magnetic field is in the range of
`250 mG` to `650 mG` (the magnitude varies depending on your geographical
location).

Some questions:

Without moving the board, what value do you see? Do you always see the same
value?

If you rotate the board, does the magnitude change? Should it change?
