//! Initialization code

#![feature(lang_items)]
#![no_std]

#[macro_use(iprint, iprintln)]
extern crate cortex_m;
extern crate f3;

pub use cortex_m::asm::{bkpt, nop};
pub use f3::hal::stm32f30x::{rcc, tim6};
pub use f3::led::Leds;
use cortex_m::itm;
use cortex_m::peripheral::ITM;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{self, TIM6, RCC};

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

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(
    args: ::core::fmt::Arguments,
    file: &'static str,
    line: u32,
    col: u32,
) -> ! {
    let itm = &mut *ITM::ptr();

    itm::write_str(&mut itm.stim[0], "PANIC at '");
    itm::write_fmt(&mut itm.stim[0], args);
    iprintln!(&mut itm.stim[0], "', {}:{}:{}", file, line, col);

    cortex_m::asm::bkpt();

    loop {}
}
