# Reverse a string

Alright, next let's make the server more interesting by having it respond to the client with the
reverse of the text that they sent. The server will respond to the client every time they press the
ENTER key. Each server response will be in a new line.

This time you'll need a buffer; you can use [`heapless::Vec`]. Here's the starter code:

[`heapless::Vec`]: https://docs.rs/heapless/0.5.6/heapless/struct.Vec.html

``` rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use rtt_target::rprintln;
use nrf51_hal as hal;
use heapless::{consts, Vec};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();

    let p0 = hal::gpio::p0::Parts::new(p.GPIO);
    let rxd = p0.p0_25.into_floating_input().degrade();
    let txd = p0.p0_24.into_push_pull_output(hal::gpio::Level::Low).degrade();

    let pins = hal::uart::Pins {
        rxd,
        txd,
        cts: None,
        rts: None
    };

    let mut uart = hal::Uart::new(p.UART0, pins, hal::uart::Parity::EXCLUDED, hal::uart::Baudrate::BAUD115200);

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
        buffer.clear();

        // TODO Receive a user request. Each user request ends with ENTER
        // NOTE `buffer.push` returns a `Result`. Handle the error by responding
        // with an error message.

        // TODO Send back the reversed string
    }
}
```
