# Receive a single byte

So far we can send data from the microcontroller to your computer. It's time to try the opposite: receiving
data from your computer. Luckily `embedded-hal` has again got us covered with this one:

``` rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
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

    loop {
        let byte = nb::block!(serial.read()).unwrap();
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
