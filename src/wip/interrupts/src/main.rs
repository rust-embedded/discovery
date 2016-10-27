#![feature(asm)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use pg::led::LEDS;
use pg::peripheral;

#[export_name = "main"]
#[inline(never)]
pub fn main() -> ! {
    let usart1 = unsafe { peripheral::usart1_mut() };

    loop {
        while !usart1.isr.read().rxne() {}

        let byte = usart1.rdr.read().rdr() as u8;
        usart1.tdr.write(|w| w.tdr(u16::from(byte)));
    }
}

#[export_name = "_tim7"]
pub extern "C" fn interrupt_handler() {
    unsafe { bkpt!() }
}
