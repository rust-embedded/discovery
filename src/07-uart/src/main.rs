#![deny(unsafe_code)]
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

    block!(uart.write(b'X')).ok();

    loop {}
}
