//! Playground

#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(bkpt, iprint, iprintln)]
extern crate f3;

#[doc(hidden)]
pub use f3::itm;

/// Low level access to peripherals
pub mod peripheral {
    pub use f3::peripheral::{gpioe, gpioe_mut};
}

#[export_name = "_init"]
#[doc(hidden)]
pub unsafe fn init() {
    f3::itm::init();
    f3::led::init();
}
