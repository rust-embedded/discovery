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