# My solution

```rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use nrf51_hal as hal;
use hal::prelude::*;
use heapless::{consts, Vec};
use nb::block;
use core::fmt::Write;

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

        loop {
            // We assume that the receiving cannot fail
            let byte = block!(uart.read()).unwrap();

            if buffer.push(byte).is_err() {
                writeln!(&mut uart, "error: buffer full").unwrap();
                break;
            }

            if byte == 13 {
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    block!(uart.write(*byte)).ok();
                }
                break;
            }
        }
    }
}
```
