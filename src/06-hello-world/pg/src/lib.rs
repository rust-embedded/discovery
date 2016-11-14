//! Playground

#![feature(asm)]
#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(bkpt, iprint, iprintln)]
#[macro_use]
extern crate f3;

#[doc(hidden)]
pub use f3::itm;

#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    f3::itm::init();
}

#[doc(hidden)]
#[export_name = "_default_exception_handler"]
pub unsafe extern "C" fn exception_handler() {
    bkpt!();

    loop {}
}
