# LED compass

In this section, we'll implement a compass using the LEDs on the F3. Like proper compasses, our LED
compass must point north somehow. It will do that by turning on one of its eight LEDs; the on LED
should point towards north.

Magnetic fields have both a magnitude, measured in Gauss or Teslas, and a *direction*. The
magnetometer on the F3 measures both the magnitude and the direction of an external magnetic field
but it reports back the *decomposition* of said field along *its axes*.

See below, the magnetometer has three axes associated to it.

<p align="center">
<img height=480 title="Magnetometer axes" src="../assets/f3-lsm303dlhc.png">
</p>

Only the X and Y axes are shown above. The Z axis is pointing "out" of your screen.

Let's get familiar with the readings of the magnetometer by running the following starter code:

``` rust
{{#include src/main.rs}}
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
I16x3 { x: 45, y: 194, z: -3 }
I16x3 { x: 46, y: 195, z: -8 }
I16x3 { x: 47, y: 197, z: -2 }
```

Now rotate the board 90 degrees while keeping it parallel to the ground. What X, Y and Z values do
you see this time? Then rotate it 90 degrees again. What values do you see?
