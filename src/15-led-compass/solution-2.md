# Solution 2

``` rust
#![deny(unsafe_code)]
#![no_std]

extern crate aux15;
extern crate m;

// you'll find this useful ;-)
use core::f32::consts::PI;

// this trait provides the `atan2` method
use m::Float;

use aux15::prelude::*;
use aux15::{Direction, I16x3};

fn main() {
    let (mut leds, mut lsm303dlhc, mut delay, _itm) = aux15::init();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let theta = (y as f32).atan2(x as f32); // radians

        let dir = if theta < -7. * PI / 8. {
            Direction::North
        } else if theta < -5. * PI / 8. {
            Direction::Northwest
        } else if theta < -3. * PI / 8. {
            Direction::West
        } else if theta < -PI / 8. {
            Direction::Southwest
        } else if theta < PI / 8. {
            Direction::South
        } else if theta < 3. * PI / 8. {
            Direction::Southeast
        } else if theta < 5. * PI / 8. {
            Direction::East
        } else if theta < 7. * PI / 8. {
            Direction::Northeast
        } else {
            Direction::North
        };

        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();

        delay.delay_ms(100_u8);
    }
}
```
