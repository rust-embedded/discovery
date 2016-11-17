# Solution 2

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate m;

#[macro_use]
extern crate pg;

use core::f32::consts::PI;

use m::Float;
use pg::I16x3;
use pg::led::Direction;
use pg::{delay, led, lsm303dlhc};

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    loop {
        let I16x3 { x, y, .. } = lsm303dlhc::magnetic_field();

        let theta = (y as f32).atan2(x as f32);

        let dir = if theta < -7. * PI / 8. {
            Direction::North
        } else if theta < -5. * PI / 8. {
            Direction::NorthWest
        } else if theta < -3. * PI / 8. {
            Direction::West
        } else if theta < -PI / 8. {
            Direction::SouthWest
        } else if theta < PI / 8. {
            Direction::South
        } else if theta < 3. * PI / 8. {
            Direction::SouthEast
        } else if theta < 5. * PI / 8. {
            Direction::East
        } else if theta < 7. * PI / 8. {
            Direction::NorthEast
        } else {
            Direction::North
        };

        led::all_off();
        dir.on();

        delay::ms(100);
    }
}
```
