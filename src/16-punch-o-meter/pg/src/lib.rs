//! Playground

#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(bkpt, iprint, iprintln)]
extern crate f3;

pub use f3::I16x3;
pub use f3::{delay, led, lsm303dlhc, time};

#[doc(hidden)]
pub use f3::itm;

#[export_name = "_init"]
pub unsafe fn init() {
    f3::delay::init();
    f3::fpu::init();
    f3::itm::init();
    f3::led::init();
    f3::lsm303dlhc::init();
    f3::time::init();
}
