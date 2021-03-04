# Receive a single byte

So far we have sending of data from the microcontroller to your computer working. It's time to try the opposite: receiving
data from your computer. Luckily `embedded-hal` again got us covered with this one:

``` rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use rtt_target::rprintln;
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

    loop {
        let byte = block!(uart.read()).unwrap();
        rprintln!("{}", byte);
    }
}
```

The only part that changed, compared to our send byte program, is the loop
at the end of `main()`. Here we use the `read()` function, provided by `embedded-hal`,
in order to wait until a byte is available and read it. Then we print that byte
into our RTT debugging console to see whether stuff is actually arriving.

Note that if you flash this program and start typing characters inside `minicom` to
send them to your microcontroller you'll only be able to see numbers inside your
RTT console since we are not converting the `u8` we received into an actual `char`.
Since the conversion from `u8` to `char` is quite simple, I'll leave this task to
you if you really do want to see the characters inside the RTT console.
