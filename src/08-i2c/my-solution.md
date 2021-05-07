# My solution

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use nrf51_hal as hal;
use hal::prelude::*;
use lsm303agr::{AccelOutputDataRate, MagOutputDataRate, Lsm303agr};
use heapless::{consts, Vec, String};
use nb::block;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();

    let p0 = hal::gpio::p0::Parts::new(p.GPIO);
    let scl = p0.p0_00.into_floating_input().degrade();
    let sda = p0.p0_30.into_floating_input().degrade();
    let rxd = p0.p0_25.into_floating_input().degrade();
    let txd = p0.p0_24.into_push_pull_output(hal::gpio::Level::Low).degrade();

    let i2c_pins = hal::twi::Pins {
        scl,
        sda,
    };

    let uart_pins = hal::uart::Pins {
        rxd,
        txd,
        cts: None,
        rts: None
    };

    // Use a frequency of 100 khz for the bus
    let i2c = hal::twi::Twi::new(p.TWI0, i2c_pins, hal::pac::twi0::frequency::FREQUENCY_A::K100);
    let mut uart = hal::Uart::new(p.UART0, uart_pins, hal::uart::Parity::EXCLUDED, hal::uart::Baudrate::BAUD115200);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        let mut buffer: Vec<u8, consts::U32> = Vec::new();

        loop {
            let byte = block!(uart.read()).unwrap();

            if buffer.push(byte).is_err() {
                write!(&mut uart, "error: buffer full\r\n").unwrap();
                break;
            }

            if byte == 13 {
                break;
            }
        }

        let command_string = String::from_utf8(buffer).unwrap();
        if command_string.as_str().trim() == "accelerometer" {
            while !sensor.accel_status().unwrap().xyz_new_data  {
            }

            let data = sensor.accel_data().unwrap();
            write!(&mut uart, "Accelerometer: x {} y {} z {}\r\n", data.x, data.y, data.z).unwrap();
        } else if command_string.as_str().trim() == "magnetometer" {
            while !sensor.mag_status().unwrap().xyz_new_data  {
            }

            let data = sensor.mag_data().unwrap();
            write!(&mut uart, "Magnetometer: x {} y {} z {}\r\n", data.x, data.y, data.z).unwrap();
        } else {
            write!(&mut uart, "error: command not detected\r\n").unwrap();
        }
    }
}
```
