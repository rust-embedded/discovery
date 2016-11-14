# Calibration

If we rotate the board, the direction of the Earth's magnetic field with respect
to the magnetometer should change but its magnitude should not! Yet, the
magnetometer indicates that the magnitude of the magnetic field changes as the
board rotates.

Why's that the case? Turns out the magnetometer needs to be calibrated to return
the correct answer.

The calibration involves quite a bit of math (matrices) so we won't cover right
now but this [Application Note] describes the procedure if you are interested.
Instead, what we'll do in this section is "visualize" how off we are.

[Application Note]: http://cache.freescale.com/files/sensors/doc/app_note/AN4246.pdf

Let's try this experiment: Let's record the readings of the magnetometer while
we slowly rotate the board along the Z axis while keeping the board horizontal
in the XY plane. We'll use the `iprintln` macro to format the readings as Tab
Separated Values (TSV).

``` rust
pub fn main() -> ! {
    loop {
        let I16x3 { x, y, z } = lsm303dlhc::magnetic_field();

        iprintln!("{}\t{}\t{}", x, y, z);

        delay::ms(100);
    }
}
```

You should get an output in the console that looks like this:

```
-331	-114	-2
-325	-147	12
-321	-169	10
-307	-209	7
-305	-216	3
-298	-236	8
-293	-243	5
-284	-257	2
-281	-263	1
-270	-278	9
```

You can pipe that to a file using:

```
# Careful! Exit any running GDB session and any `itmdump` instance that may be
# using `itm.txt`
$ itmdump itm.txt > emf.txt
```

Then import that TSV file into a spreadsheet program and plot the first two
columns as a scatter plot.

<p align="center">
<img title="Earth's magnetic field" src="assets/earth-magnetic-field.png">
</p>

If you rotated the board on a flat horizontal surface, the Z component of the
magnetic field should have remained relatively constant and this plot should
have been a circle (not a ellipse) centered at the origin. Severe deviations
from that indicate that the magnetometer needs to be calibrated.

Take home message: Don't just trust the reading of a sensor. Verify it's
outputting sensible values. If it's not, then calibrate it.
