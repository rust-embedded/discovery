//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::asm::{bkpt, nop};
pub use cortex_m_rt::entry;
pub use stm32f3::stm32f303::{rcc, tim6, RCC, TIM6};
pub use stm32f3_discovery::switch_hal;

use stm32f3_discovery::{
    leds::Leds,
    stm32f3xx_hal::{prelude::*, stm32},
};

pub fn init() -> (
    Leds,
    &'static rcc::RegisterBlock,
    &'static tim6::RegisterBlock,
) {
    let p = stm32::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let mut gpioe = p.GPIOE.split(&mut rcc.ahb);

    let leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    (leds, unsafe { &*RCC::ptr() }, unsafe { &*TIM6::ptr() })
}
