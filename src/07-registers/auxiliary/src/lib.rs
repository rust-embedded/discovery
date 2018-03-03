//! Initialization code

#![feature(lang_items)]
#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(iprint, iprintln)]
#[macro_use(iprint, iprintln)]
extern crate cortex_m;
extern crate f3;

pub use cortex_m::asm::bkpt;
pub use cortex_m::peripheral::ITM;
pub use f3::hal::stm32f30x::gpioc;
use cortex_m::itm;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{self, GPIOE};
use f3::led::Leds;

#[inline(never)]
pub fn init() -> (ITM, &'static gpioc::RegisterBlock) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    (cp.ITM, unsafe { &*GPIOE::ptr() })
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
