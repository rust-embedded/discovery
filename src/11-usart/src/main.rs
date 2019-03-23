#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // 1文字送信します
    usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));

    loop {}
}
