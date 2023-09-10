# Take 1

实现LED指南针的最简单方法是什么，即使它并不完美？

对于初学者，我们只关心磁场的X和Y分量，因为当您查看指南针时，您总是将它保持在水平位置，因此指南针位于XY平面上。

<p>
<img class="white_bg" title="Quadrants" src="../assets/quadrants.png">
</p>

如果我们只看X和Y分量的符号，我们就可以确定磁场属于哪个象限。在的问题当然是4个象限代表哪个方向(北、东北等)。
为了弄清楚这一点，我们可以旋转micro:bit并观察当我们指向另一个方向时象限如何变化。

经过一番实验，我们可以发现，如果我们将micro:bit指向例如东北方向，X和Y分量始终为正。
根据这些信息，您应该能够确定其他象限代表的方向。

一旦你弄清楚象限和方向之间的关系，你应该能够从下面完成模板。

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod calibration;
use crate::calibration::calc_calibration;
use crate::calibration::calibrated_measurement;

mod led;
use led::Direction;

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
