# My solution

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
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
    AccelScale, AccelOutputDataRate, Lsm303agr,
};

use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;
use nb::Error;

#[entry]
fn main() -> ! {
    const THRESHOLD: f32 = 0.5;

    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut countdown = Timer::new(board.TIMER0);
    let mut delay = Timer::new(board.TIMER1);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    // Allow the sensor to measure up to 16 G since human punches
    // can actually be quite fast
    sensor.set_accel_scale(AccelScale::G16).unwrap();

    let mut max_g = 0.;
    let mut measuring = false;

    loop {
        while !sensor.accel_status().unwrap().xyz_new_data {}
        // x acceleration in g
        let g_x = sensor.accel_data().unwrap().x as f32 / 1000.0;

        if measuring {
            // Check the status of our contdown
            match countdown.wait() {
                // countdown isn't done yet
                Err(Error::WouldBlock) => {
                    if g_x > max_g {
                        max_g = g_x;
                    }
                },
                // Countdown is done
                Ok(_) => {
                    // Report max value
                    rprintln!("Max acceleration: {}g", max_g);

                    // Reset
                    max_g = 0.;
                    measuring = false;
                },
                // Since the nrf52 and nrf51 HAL have Void as an error type
                // this path cannot occur, as Void is an empty type
                Err(Error::Other(_)) => {
                    unreachable!()
                }
            }
        } else {
            // If acceleration goes above a threshold, we start measuring
            if g_x > THRESHOLD {
                rprintln!("START!");

                measuring = true;
                max_g = g_x;
                // The documentation notes that the timer works at a frequency
                // of 1 Mhz, so in order to wait for 1 second we have to
                // set it to 1_000_000 ticks.
                countdown.start(1_000_000_u32);
            }
        }
        delay.delay_ms(20_u8);
    }
}
```
