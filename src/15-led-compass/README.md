# LED compass

In this section, we'll implement a compass using the LEDs on the F3. Like proper
compasses, our LED compass must point north somehow. It will do that by turning
on one of its eight LEDs; that LED should point towards north.

Magnetic fields have both a magnitude, measured in Gauss or Teslas, and a
*direction*. The magnetometer on the F3 measures both the magnitude and the
direction of an external magnetic field but it reports back the *decomposition*
of said field along *its axes*.

See below, the magnetometer has three axes associated to it.

<p align="center">
<img height=480 title="Magnetometer axes" src="assets/f3-lsm303dlhc.png">
</p>

Only the X and Y axes are shown above. The Z axis is pointing "out" of your
screen.

Let's get familiar with the readings of the magnetometer by running the
following starter code:

``` rust
#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    loop {
        iprintln!("{:?}", lsm303dlhc::magnetic_field());
        delay::ms(1_000);
    }
}
```

This `lsm303dlhc` module provides high level API over the LSM303DLHC. Under the
hood it does the same I2C routine that you implemented in the last section but
it reports the X, Y and Z values in a `I16x3` struct instead of a tuple.

Locate where north is at your current location. Then rotate the board such that
it's aligned "towards north": its `North` LED should be pointing towards north.

Now run the starter code and observe the output. What X, Y and Z values do you
see?

```
# itmdump's console
(..)
I16x3 { x: 322, y: 24, z: -11 }
I16x3 { x: 323, y: 24, z: -13 }
I16x3 { x: 324, y: 23, z: -12 }
I16x3 { x: 324, y: 20, z: -10 }
```

Now rotate the board 180 degrees. What X, Y and Z values do you see this time?

Finally, rotate the board 90 degrees clockwise. The `East` LED should be
pointing north this time. What X, Y and Z values do you see this time?
