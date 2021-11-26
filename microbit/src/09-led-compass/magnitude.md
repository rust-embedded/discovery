# Magnitude

We have been working with the direction of the magnetic field but what is its real magnitude?
According to the documentation about the [`mag_data()`] function the `x` `y` `z` values we are
getting are in nanotesla. That means the only thing we have to compute in order to get the
magnitude of the magnetic field in nanotesla is the magnitude of the 3D vector that our `x` `y` `z`
values describe. As you might remember from school this is simply:

``` rust
// core doesn't have this function yet so we use libm, just like with
// atan2f from before.
use libm::sqrtf;
let magnitude = sqrtf(x * x + y * y + z * z);
```

[`mag_data()`]: https://docs.rs/lsm303agr/0.2.2/lsm303agr/struct.Lsm303agr.html#method.mag_data


Putting all this together in a program:

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod calibration;
use crate::calibration::calc_calibration;
use crate::calibration::calibrated_measurement;

use libm::sqrtf;

use microbit::{display::blocking::Display, hal::Timer};

#[cfg(feature = "v1")]
use microbit::{hal::twi, pac::twi0::frequency::FREQUENCY_A};

#[cfg(feature = "v2")]
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};

use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};

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
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let calibration = calc_calibration(&mut sensor, &mut display, &mut timer);
    rprintln!("Calibration: {:?}", calibration);
    rprintln!("Calibration done, entering busy loop");
    loop {
        while !sensor.mag_status().unwrap().xyz_new_data {}
        let mut data = sensor.mag_data().unwrap();
        data = calibrated_measurement(data, &calibration);
        let x = data.x as f32;
        let y = data.y as f32;
        let z = data.z as f32;
        let magnitude = sqrtf(x * x + y * y + z * z);
        rprintln!("{} nT, {} mG", magnitude, magnitude/100.0);
    }
}
```

This program will report the magnitude (strength) of the magnetic field in nanotesla (`nT`) and milligauss (`mG`). The
magnitude of the Earth's magnetic field is in the range of `250 mG` to `650 mG` (the magnitude
varies depending on your geographical location) so you should see a value in that range or close to
that range -- I see a magnitude of around `340 mG`.

Some questions:

Without moving the board, what value do you see? Do you always see the same value?

If you rotate the board, does the magnitude change? Should it change?
