# Reverse a string

Alright, next let's make the server more interesting by having it respond to the client with the
reverse of the text that they sent. The server will respond to the client every time they press the
ENTER key. Each server response will be in a new line.

This time you'll need a buffer; you can use [`heapless::Vec`]. Here's the starter code:

[`heapless::Vec`]: https://docs.rs/heapless/latest/heapless/struct.Vec.html

``` rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use core::fmt::Write;
use heapless::Vec;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

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
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        // TODO Receive a user request. Each user request ends with ENTER
        // NOTE `buffer.push` returns a `Result`. Handle the error by responding
        // with an error message.

        // TODO Send back the reversed string
    }
}
```
