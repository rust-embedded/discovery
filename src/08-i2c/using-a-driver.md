# Using a driver

As we already discussed in chapter 5 `embedded-hal` provides abstractions
which can be used to write platform independent code that can interact with
hardware. In fact all the methods we have used to interact with hardware
in chapter 7 and up until now in chapter 8 were from traits, defined by `embedded-hal`.
Now we'll make actual use of the traits defined by `embedded-hal` provides for the first time.

Since it would be pointless to just implement a driver for our LSM303AGR for every platform
embedded Rust supports (and new ones that might eventually pop up) one can write a driver
that simply consume generic types that implement `embedded-hal` traits in order to provide
a platform agnostic version of a driver. Luckily for us this has already been done in the
[lsm303agr] crate. Hence reading the actual accelerometer and magnetometer values will now
be basically a plug and play experience + reading a bit of documentation. In fact the crates.io
page already provides us with everything we need to know in order to read accelerometer data but using a Raspberry Pi. We'll
just have to adapt it to our chip:

[lsm303agr]: https://crates.io/crates/lsm303agr

```rust
use linux_embedded_hal::I2cdev;
use lsm303agr::{AccelOutputDataRate, Lsm303agr};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Lsm303agr::new_with_i2c(dev);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            println!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
        }
    }
}
```

Because we already know how to create an instance of an object that implements
the `embedded-hal` I2C traits from the last page this is quite trivial:

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use nrf51_hal as hal;
use hal::prelude::*;
use lsm303agr::{AccelOutputDataRate, Lsm303agr};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();

    let p0 = hal::gpio::p0::Parts::new(p.GPIO);
    let scl = p0.p0_00.into_floating_input().degrade();
    let sda = p0.p0_30.into_floating_input().degrade();

    let pins = hal::twi::Pins {
        scl,
        sda,
    };

    // Use a frequency of 100 khz for the bus
    let i2c = hal::twi::Twi::new(p.TWI1, pins, hal::pac::twi0::frequency::FREQUENCY_A::K100);

	// Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
			// RTT instead of normal print
            rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
        }
    }
}
```

Just like the last snippet you should just be able to try this out with `cargo-embed`.
Furthermore if you (physically) move around your micro:bit a little bit you should see the
acceleration numbers that are being printed change.
