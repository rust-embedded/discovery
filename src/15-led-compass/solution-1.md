# Solution 1

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

        let dir = match (x > 0, y > 0) {
            (false, false) => Direction::NorthWest,
            (false, true) => Direction::NorthEast,
            (true, false) => Direction::SouthWest,
            (true, true) => Direction::SouthEast,
        };

        led::all_off();
        dir.on();

        delay::ms(100);
    }
}
```
