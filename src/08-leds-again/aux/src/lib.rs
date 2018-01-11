//! Initialization code

#![feature(lang_items)]
#![no_std]

#[macro_use(iprint, iprintln)]
extern crate cortex_m;
extern crate f3;

pub use cortex_m::asm::bkpt;
pub use f3::hal::stm32f30x::{gpioc, rcc};
use cortex_m::itm;
use cortex_m::peripheral::ITM;
use f3::hal::stm32f30x::{self, GPIOE, RCC};

pub fn init() -> (&'static gpioc::RegisterBlock, &'static rcc::RegisterBlock) {
    // restrict access to the other peripherals
    (stm32f30x::Peripherals::take().unwrap());

    unsafe { (&*GPIOE::ptr(), &*RCC::ptr()) }
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
