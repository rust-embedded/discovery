//! Initialization code

#![feature(panic_implementation)]
#![no_std]

#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate f3;

use core::panic::PanicInfo;

use cortex_m::asm;
pub use cortex_m::asm::bkpt;
pub use cortex_m::peripheral::ITM;
use cortex_m_rt::ExceptionFrame;

pub fn init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();

    p.ITM
}

#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    let itm = unsafe { &mut *ITM::ptr() };

    iprintln!(&mut itm.stim[0], "{}", info);

    cortex_m::asm::bkpt();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {}
}

exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    loop {}
}
