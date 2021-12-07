# Punch-o-meter

In this section we'll be playing with the accelerometer that's in the board.

What are we building this time? A punch-o-meter! We'll be measuring the power of your jabs. Well,
actually the maximum acceleration that you can reach because acceleration is what accelerometers
measure. Strength and acceleration are proportional though so it's a good approximation.

The accelerometer is also built inside the LSM303DLHC package. And just like the magnetometer, it
can also be accessed using the I2C bus. It also has the same coordinate system as the magnetometer.
Here's the coordinate system again:

<p align="center">
<img height=480 title="Magnetometer axes" src="../assets/f3-lsm303dlhc.png">
</p>

Just like in the previous unit, we'll be using a high level API to directly get the sensor readings
in a nicely packaged `struct`.
