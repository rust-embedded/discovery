//! Initialization code

#![deny(warnings)]
#![no_std]

use panic_itm as _; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;

pub use stm32f3::stm32f303::{self, gpioc::RegisterBlock};
pub use stm32f3_discovery::stm32f3xx_hal::pac::GPIOE;
pub use stm32f3_discovery::{leds::Leds, stm32f3xx_hal};

use stm32f3xx_hal::prelude::*;
pub use stm32f3xx_hal::stm32;

#[inline(never)]
pub fn init() -> (ITM, &'static RegisterBlock) {
    let device_periphs = stm32::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    // initialize user leds
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let _leds = Leds::new(
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

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    (core_periphs.ITM, unsafe { &*stm32f303::GPIOE::ptr() })
}
