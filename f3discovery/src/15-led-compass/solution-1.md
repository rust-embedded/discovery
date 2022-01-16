<!-- # Solution 1 -->

# 解答例1

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

        // 磁場がどの象限に属するか決めるため、XとY要素の符号を見ます。
        let dir = match (x > 0, y > 0) {
            // 第I象限
            (true, true) => Direction::Southeast,
            // 第II象限
            (false, true) => Direction::Northeast,
            // 第III象限
            (false, false) => Direction::Northwest,
            // 第IV象限
            (true, false) => Direction::Southwest,
        };

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(1_000_u16);
    }
}
```
