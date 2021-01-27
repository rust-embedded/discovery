# Naive approach and `writeln!`

## Naive approach

You probably came up with a program similar to the following:

```rs
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use nrf51_hal as hal;
use hal::prelude::*;
use nb::block;

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

    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        block!(uart.write(*byte)).ok();
    }

    loop {}
}
```

While this is a perfectly valid implementation, at some point
you might want to have all the nice perks of `println!` such
as argument formatting and so on. If you are wondering how to do that, read on.

## `writeln!` and `core::fmt::Write`
The `core::fmt::Write` trait allows us to use any struct that implements
it in basically the same way as we use `println!` in the `std` world.
In this case the `Uart` struct from the `nrf` HAL does implement `core::fmt::Write`
so we can refactor our previous program into this:

```rs
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use nrf51_hal as hal;
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
    writeln!(&mut uart, "The quick brown fox jumps over the lazy dog.").unwrap();

    loop {}
}
```

If you were to flash this program onto your micro:bit, you'll
see that it is functionally equivalent to the iterator-based
program you came up with.
