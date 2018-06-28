#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate aux11;
#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate heapless;

entry!(main);

fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // Send a single character
    usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));

    loop {}
}
