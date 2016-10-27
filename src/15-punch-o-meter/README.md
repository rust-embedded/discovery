# Punch-o-meter

In this section we'll be playing with the accelerometer that's in the board.

What are we build this time? A punch-o-meter! We'll be measuring the power of
your jabs. Well, actually the maximum acceleration that you can reach because
acceleration is what accelerometers measure. Strength and acceleration are
proportional though so it's a good approximation.

The accelerometer is also built inside the LSM303DLHC package. And just like the
magnetometer, it can also be accessed using the I2C bus. It also has the same
coordinate system as the magnetometer. Here's the coordinate system again:

<p align="center">
<img height=480 title="Magnetometer axes" src="assets/f3-lsm303dlhc.png">
</p>

Just like in the previous unit, we'll be using a high level API to directly get
the sensor reading in a nicely packaged `struct`.

The weirdest thing is that the accelerometer is measuring an "acceleration" even
though it's not moving. An a non-zero acceleration implies movement (or does
that depends on the frame of reference?).

OK, so at rest, the acceleration is pointing upwards (towards the ceiling)?
Hmm, doesn't the acceleration of gravity points downwards (towards the floor)?
What's going on?

The best explanation that I know for this is: This measurement you are seeing is
an "artifact"" of how the accelerometer, itself, is implemented.

Let's talk about that.

This particular accelerometer is a *MEMS* sensor where MEMS stands for "Micro
Electro Mechanical System"". That means there is a mechanical component inside
of it and that component is a *cantilever beam* (no, not a laser), a microscopic
one (hence the Micro in the name).

> **TODO** picture of a cantilever beam

With me so far? OK. The accelerometer measures the acceleration indirectly by
measuring the deflection of the beam. If the beam is not deflected, that is it's
straight, then the measured acceleration is zero otherwise the acceleration is
non-zero.

As for the signs of the acceleration: If you accelerate (suddenly move) the beam
upwards (towards the ceiling). How it will deflect? It will deflect downwards
(towards the floor). Why? Because of inertia: its free end "wants to" stay down
(where it was). If you do the opposite and move the beam downwards, the beam
will deflect upwards.

So: deflection downwards equals positive acceleration and deflection upwards
equals negative acceleration.

Makes sense so far? OK, Back to the gravity. While resting (not in movement),
the beam will be deflected downwards because of its own weight. That's
interpreted as positive acceleration by the sensor.

That's why the acceleration of gravity appears to be backwards.

Yeah, physics is hard.

Question for the reader: What acceleration will the accelerometer measure when
it's in "free fall"? Please, don't actually try this scenario, as in let your
board fall, because you may damage it.

## Magnitude

The accelerometer is configured to operate in the `[-2g, +2g]` range. Let's
convert the unitless `i16` value returned by the API into a real value with
proper units.

``` rust
pub extern "C" fn main() -> ! {
    loop {
        const FACTOR: f32 = 2. / ((1 << 15) as f32);

        let I16x3 { x, y, z } = lsm303dlhc::acceleration();

        let x = f32::from(x) * FACTOR;
        let y = f32::from(y) * FACTOR;
        let z = f32::from(z) * FACTOR;

        iprintln!("{:?}", (x, y, z));

        delay::ms(1_000);
    }
}
```

## The meter



Next thing we'll have to do is detect the maximum value and report that. We'll
also need a "reset" condition to start a new measurement.
