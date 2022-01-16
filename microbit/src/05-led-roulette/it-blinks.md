# It blinks

## Delaying
Now we're going to take a brief look into delay abstractions provided by `embedded-hal`
before combining this with the GPIO abstractions from the previous chapter in order to
finally make an LED blink.

`embedded-hal` provides us with two abstractions to delay the execution of our program:
[`DelayUs`] and [`DelayMs`]. Both of them essentially work the exact same way except
that they accept different units for their delay function.

[`DelayUs`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/blocking/delay/trait.DelayUs.html
[`DelayMs`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/blocking/delay/trait.DelayMs.html

Inside our MCU, several so-called "timers" exist. They can do various things regarding time for us,
including simply pausing the execution of our program for a fixed amount of time. A very
simple delay-based program that prints something every second might for example look like this:

```rs
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    loop {
        timer.delay_ms(1000u16);
        rprintln!("1000 ms passed");
    }
}
```

Note that we changed our panic implementation from `panic_halt` to
`panic_rtt_target` here. This will require you to uncomment the two
RTT lines from `Cargo.toml` and comment the `panic-halt` one out,
since Rust only allows one panic implementation at a time.

In order to actually see the prints we have to change `Embed.toml` like this:
```
[default.general]
# chip = "nrf52833_xxAA" # uncomment this line for micro:bit V2
# chip = "nrf51822_xxAA" # uncomment this line for micro:bit V1

[default.reset]
halt_afterwards = false

[default.rtt]
enabled = true

[default.gdb]
enabled = false
```

And now after putting the code into `src/main.rs` and another quick `cargo embed` (again with the same flags you used before)
you should see "`1000 ms passed`" being sent to your console every second from your MCU.

## Blinking

Now we've arrived at the point where we can combine our new knowledge about GPIO and delay abstractions
in order to actually make an LED on the back of the micro:bit blink. The resulting program is really just
a mash-up of the one above and the one that turned an LED on in the last section and looks like this:

```rs
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();
    let mut row1 = board.display_pins.row1;

    loop {
        row1.set_low().unwrap();
        rprintln!("Dark!");
        timer.delay_ms(1_000_u16);
        row1.set_high().unwrap();
        rprintln!("Light!");
        timer.delay_ms(1_000_u16);
    }
}
```

And after putting the code into `src/main.rs` and a final `cargo embed` (with the proper flags)
you should see the LED we light up before blinking as well as a print, every time the LED changes from off to on and vice versa.
