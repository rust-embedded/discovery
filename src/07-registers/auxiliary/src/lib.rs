//! Initialization code

#![deny(warnings)]
#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
use f3::{
    hal::{
        prelude::*,
        stm32f30x::gpioc,
        stm32f30x::{self, GPIOE},
    },
    led::Leds,
};

#[inline(never)]
pub fn init() -> (ITM, &'static gpioc::RegisterBlock) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    (cp.ITM, unsafe { &*GPIOE::ptr() })
}
