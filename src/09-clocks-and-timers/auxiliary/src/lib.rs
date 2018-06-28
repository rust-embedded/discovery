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
pub use cortex_m::asm::{bkpt, nop};
use cortex_m::peripheral::ITM;
use cortex_m_rt::ExceptionFrame;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{self, TIM6, RCC};
pub use f3::hal::stm32f30x::{rcc, tim6};
pub use f3::led::Leds;

pub fn init() -> (
    Leds,
    &'static rcc::RegisterBlock,
    &'static tim6::RegisterBlock,
) {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let leds = Leds::new(p.GPIOE.split(&mut rcc.ahb));

    (leds, unsafe { &*RCC::ptr() }, unsafe { &*TIM6::ptr() })
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
