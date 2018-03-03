//! Initialization code

#![feature(lang_items)]
#![feature(macro_reexport)]
#![no_std]

#[macro_use(iprintln, iprint)]
#[macro_reexport(iprintln, iprint)]
extern crate cortex_m;
extern crate f3;

pub use cortex_m::asm::bkpt;
pub use cortex_m::peripheral::ITM;
use cortex_m::itm;

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
