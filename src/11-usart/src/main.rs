#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // Send a single character
    usart1
        .tdr
        .write(|w| w.tdr().bits(u16::from(b'X')) );

    loop {}
}
