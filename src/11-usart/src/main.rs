#![deny(unsafe_code)]
#![no_std]

extern crate aux11;

fn main() {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // Send a single character
    usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));
}
