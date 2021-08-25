# Take 1

What's the simplest way in which we can implement the LED compass, even if it's not perfect?

For starters, we'd only care about the X and Y components of the magnetic field because when you
look at a compass you always hold it in horizontal position and thus the compass is in the XY plane.

<p align="center">
<img class="white_bg" title="Quadrants" src="../assets/quadrants.png">
</p>

If we only looked at the signs of the X and Y components we could determine to which quadrant the
magnetic field belongs to. Now the question of course is which direction (north, north east, etc.)
do the 4 quadrants represent. In order to figure this out we can just rotate the micro:bit and observe
how the quadrant changes whenever we point in another direction.

After experimenting a bit we can find out that if we point the micro:bit in e.g. south east direction,
both the X and the Y component are always positive. Based on this information you should be able to
figure out which direction the other quadrants represent.

Once you figured out the relation between quadrant and direction you should be able to
complete the template from below.

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

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
use led::Direction;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data  {}
        let data = sensor.mag_data().unwrap();

        let dir = match (data.x > 0, data.y > 0) {
            // Quadrant ???
            (true, true) => Direction::NorthEast,
            // Quadrant ???
            (false, true) => panic!("TODO"),
            // Quadrant ???
            (false, false) => panic!("TODO"),
            // Quadrant ???
            (true, false) => panic!("TODO"),
        };

        // use the led module to turn the direction into an LED arrow
        // and the led display functions from chapter 5 to display the
        // arrow
    }
}
```