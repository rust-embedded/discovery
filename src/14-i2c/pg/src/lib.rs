//! Playground

#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(bkpt, iprint, iprintln)]
extern crate f3;

pub use f3::{delay, led, time};

#[doc(hidden)]
pub use f3::itm;

/// Low level access to peripherals
pub mod peripheral {
    pub use f3::peripheral::{i2c1, i2c1_mut};
}

#[export_name = "_init"]
pub unsafe fn init() {
    f3::delay::init();
    f3::fpu::init();
    f3::itm::init();
    f3::led::init();
    f3::lsm303dlhc::init();
    f3::time::init();
}
