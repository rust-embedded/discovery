//! Initialization code

#![feature(lang_items)]
#![feature(use_extern_macros)]
#![no_std]

extern crate cortex_m;
extern crate f3;

pub use cortex_m::asm::bkpt;
use cortex_m::itm;
pub use cortex_m::peripheral::ITM;
pub use cortex_m::{iprint, iprintln};

pub fn init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();

    p.ITM
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
