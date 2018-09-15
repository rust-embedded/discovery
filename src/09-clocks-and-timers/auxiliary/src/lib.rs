//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::asm::{bkpt, nop};
pub use cortex_m_rt::entry;
pub use f3::{
    hal::stm32f30x::{rcc, tim6},
    led::Leds,
};

use f3::hal::{
    prelude::*,
    stm32f30x::{self, RCC, TIM6},
};

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
