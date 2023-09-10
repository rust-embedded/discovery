# 大小

我们一直在研究磁场的方向，但它的实际大小是多少？根据关于[`mag_data()`]函数的文档，我们得到的
`x` `y` `z`值是纳米级的。这意味着我们唯一需要计算的，就是我们的`x` `y` `z`值所描述的三维矢量的大小，
才能得到纳米级磁场的大小。正如你在学校所记得的，这很简单：

``` rust
// core doesn't have this function yet so we use libm, just like with
// atan2f from before.
use libm::sqrtf;
let magnitude = sqrtf(x * x + y * y + z * z);
```

[`mag_data()`]: https://docs.rs/lsm303agr/0.2.2/lsm303agr/struct.Lsm303agr.html#method.mag_data


将所有这些放在一个程序中：

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

该程序将报告磁场的大小 (强度) ，单位为纳米特斯拉 (`nT`) 和毫米高斯 (`mG`)。地球磁场的大小在
`250 mG` 到 `650 mG`之间（大小取决于你的地理位置）， 所以你应该看到一个在这个范围内或接近这个范围
的值——我看到大约`340 mG`。

一些问题：

如果不移动开发板，您看到了什么值？你总是看到相同的值吗？

如果旋转开发板，大小是否会改变？应该改变吗？
