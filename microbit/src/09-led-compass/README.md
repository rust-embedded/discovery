# LED compass

In this section, we'll implement a compass using the LEDs on the micro:bit. Like proper compasses, our LED
compass must point north somehow. It will do that by turning on one of its outer LEDs; the LED turned on
should point towards north.

Magnetic fields have both a magnitude, measured in Gauss or Teslas, and a *direction*. The
magnetometer on the micro:bit measures both the magnitude and the direction of an external magnetic field
but it reports back the *decomposition* of said field along *its axes*.

The magnetometer has three axes associated to it. The X and Y axes basically span the plane that is the floor.
The Z axis is pointing "out" of the floor, so upwards.

You should already be able to write a program that continuously prints the magnetometer
data on the RTT console from the [I2C chapter](../08-i2c/index.md). After you wrote that
program, locate where north is at your current location. Then line up your micro:bit with
that direction and observe how the sensor's measurements look.

Now rotate the board 90 degrees while keeping it parallel to the ground. What X, Y and Z values do
you see this time? Then rotate it 90 degrees again. What values do you see?
