#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v1")]
use embedded_io::Write;

#[cfg(feature = "v2")]
use embedded_hal_nb::serial::Write;

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        // Set up UART for microbit v1
        let serial = uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        serial
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        // Set up UARTE for microbit v2 using UartePort wrapper
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // Write a byte and flush
    #[cfg(feature = "v1")]
    serial.write(&[b'X']).unwrap(); // Adjusted for UART on v1, no need for nb::block!

    #[cfg(feature = "v2")]
    {
        nb::block!(serial.write(b'X')).unwrap();
        nb::block!(serial.flush()).unwrap();
    }

    loop {}
}
