# Take 1

What's the simplest way in which we can implement the LED compass? Even if it's
not perfect.

For starters, we'd only care about the X and Y components of the magnetic field
because when you look at a compass you always hold it in horizontal position
thus the compass is in the XY plane.

For example, what LED would you turn on in the following case. EMF stands for
Earth's Magnetic Field and green arrow has the direction of the EMF (it points
north).

<p align="center">
<img title="Quadrant I" src="assets/quadrant-i.png">
</p>

The `SouthEast` LED, right?

What *signs* do the X and Y components of the magnetic field have in that
scenario? Both are positive.

If we only looked at the signs of the X and Y components we could determine to
which quadrant the magnetic field belongs to.

<p align="center">
<img title="Quadrants" src="assets/quadrants.png">
</p>

In the previous example, the magnetic field was in the first quadrant (x and y
were positive) and it made sense to turn on the `SouthEast` LED. Similarly, we
could turn a different LED if the magnetic field was in a different quadrant.

Let's try that logic. Here's the starter code:

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use pg::I16x3;
use pg::led::Direction;
use pg::{delay, led, lsm303dlhc};

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    loop {
        let I16x3 { x, y, .. } = lsm303dlhc::magnetic_field();

        // Look at the signs of the X and Y components to determine in which
        // quadrant the magnetic field is
        let dir = match (x > 0, y > 0) {
            // Quadrant I
            (true, true) => Direction::SouthEast,
            // Quadrant II
            (false, true) => { /* TODO */ },
            // Quadrant III
            (false, false) => { /* TODO */ },
            // Quadrant IV
            (true, false) => { /* TODO */ },
        };

        led::all_off();
        dir.on();

        delay::ms(100);
    }
}
```

There's a `Direction` enum in the `led` module that has 8 variants named after
the cardinal points: `North`, `East`, `SouthWest`, etc. Each of these variants
represent one of the 8 LEDs in the compass. The `Direction` `enum` exposes `on`
and `off` methods like the `Led` struct does so you can use it to turn on/off
the LED that's associated to that particular cardinal point.
