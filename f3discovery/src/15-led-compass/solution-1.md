# Solution 1

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, switch_hal::OutputSwitch, Direction, I16x3};

#[entry]
fn main() -> ! {
    let (leds, mut lsm303dlhc, mut delay, _itm) = aux15::init();
    let mut leds = leds.into_array();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        // Look at the signs of the X and Y components to determine in which
        // quadrant the magnetic field is
        let dir = match (x > 0, y > 0) {
            // Quadrant I
            (true, true) => Direction::Southeast,
            // Quadrant II
            (false, true) => Direction::Northeast,
            // Quadrant III
            (false, false) => Direction::Northwest,
            // Quadrant IV
            (true, false) => Direction::Southwest,
        };

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(1_000_u16);
    }
}
```
