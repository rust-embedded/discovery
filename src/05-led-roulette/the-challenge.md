# The challenge

You are now well armed to face a challenge! Your task will be to implement the application I showed
you at the beginning of this chapter.

<p align="center">
<video src="../assets/roulette_fast.mp4" loop autoplay>
</p>

If you can't exactly see what's happening here it is in a much slower version:

<p align="center">
<video src="../assets/roulette_slow.mp4" loop autoplay>
</p>

Since working with the LED pins separately is quite annoying
(especially if you have to use basically all of them like here)
you can use the display API provided by the BSP. It works like this:

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let light_it_all = [
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
    ];

    loop {
        // Show light_it_all for 1000ms
        display.show(&mut timer, light_it_all, 1000);
        // clear the display again
        display.clear();
        timer.delay_ms(1000_u32);
    }
}
```

Equipped with this API your task basically boils down to just having
to calculate the proper image matrix and passing it into the BSP.
