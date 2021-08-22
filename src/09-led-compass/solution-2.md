# Solution 2

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use rtt_target::rprintln;
use panic_rtt_target as _;

// You'll find this useful ;-)
use core::f32::consts::PI;
use libm::atan2f;

use microbit::{
    display::blocking::Display,
    hal::Timer,
};

#[cfg(feature = "v1")]
use microbit::{
    hal::twi,
    pac::twi0::frequency::FREQUENCY_A,
};

#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{
    MagOutputDataRate, Lsm303agr,
};

mod led;
use led::{Direction, direction_to_led};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data  {}
        let data = sensor.mag_data().unwrap();

        // use libm's atan2f since this isn't in core yet
        let theta = atan2f(data.y as f32, data.x as f32);

        // Figure out the direction based on theta
        let dir = if theta < -7. * PI / 8. {
            Direction::West
        } else if theta < -5. * PI / 8. {
            Direction::NorthWest
        } else if theta < -3. * PI / 8. {
            Direction::North
        } else if theta < -PI / 8. {
            Direction::NorthEast
        } else if theta < PI / 8. {
            Direction::East
        } else if theta < 3. * PI / 8. {
            Direction::SouthEast
        } else if theta < 5. * PI / 8. {
            Direction::South
        } else if theta < 7. * PI / 8. {
            Direction::SouthWest
        } else {
            Direction::West
        };

        rprintln!("x: {}, y: {}, dir: {:?}", data.x, data.y, dir);

        display.show(&mut timer, direction_to_led(dir), 100);
    }
}
```
