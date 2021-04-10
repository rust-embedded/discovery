#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use nrf51_hal as hal;
use hal::prelude::*;

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

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
    let mut i2c = hal::twi::Twi::new(p.TWI1, pins, hal::pac::twi0::frequency::FREQUENCY_A::K100);
    let mut ac_data = [0];
    let mut ma_data = [0];

    // First write the address + register onto the bus, then read the chip's responses
    i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut ac_data).unwrap();
    i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut ma_data).unwrap();

    rprintln!("The accelerometer chip's id is: {:#b}", ac_data[0]);
    rprintln!("The magnetometer chip's id is: {:#b}", ma_data[0]);

    loop {}
}
