#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{
    rtt_init_print,
    rprintln,
};
use panic_rtt_target as _;
use embedded_hal::{
    delay::DelayNs,
    digital::OutputPin,
};
use microbit::board::Board;
use microbit::hal::timer::Timer;

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
        timer.delay_ms(1_000_u32);

        row1.set_high().unwrap();
        rprintln!("Light!");
        timer.delay_ms(1_000_u32);
    }
}
