#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use core::f32::consts::PI;
use libm::atan2f;

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

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();


    #[cfg(feature = "v1")]
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    // Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();
    loop {
        while !sensor.mag_status().unwrap().xyz_new_data  {}
        let data = sensor.mag_data().unwrap();
        let theta = atan2f(data.y as f32, data.x as f32); // in radians

        let dir = if theta < -7. * PI / 8. {
            "North"
        } else if theta < -5. * PI / 8. {
            "Northwest"
        } else if theta < -3. * PI / 8. {
            "West"
        } else if theta < -PI / 8. {
            "Southwest"
        } else if theta < PI / 8. {
            "South"
        } else if theta < 3. * PI / 8. {
            "Southeast"
        } else if theta < 5. * PI / 8. {
            "East"
        } else if theta < 7. * PI / 8. {
            "Northeast"
        } else {
            "North"
        };
        rprintln!("Magnetometer: {}", dir);
    }
}
