//! Initialization code

#![feature(panic_implementation)]
#![no_std]

#[macro_use]
extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m;
extern crate f3;

use core::panic::PanicInfo;

use cortex_m::asm;
pub use cortex_m::asm::bkpt;
use cortex_m::peripheral::ITM;
use cortex_m_rt::ExceptionFrame;
use f3::hal::stm32f30x::{self, GPIOE, RCC};
pub use f3::hal::stm32f30x::{gpioc, rcc};

pub fn init() -> (&'static gpioc::RegisterBlock, &'static rcc::RegisterBlock) {
    // restrict access to the other peripherals
    (stm32f30x::Peripherals::take().unwrap());

    unsafe { (&*GPIOE::ptr(), &*RCC::ptr()) }
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
