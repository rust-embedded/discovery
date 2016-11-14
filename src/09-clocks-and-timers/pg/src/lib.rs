//! Playground

#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(bkpt, iprint ,iprintln)]
extern crate f3;

#[doc(hidden)]
pub use f3::itm;

pub use f3::led;

/// Low level access to peripherals
pub mod peripheral {
    pub use f3::peripheral::{rcc, rcc_mut, tim7, tim7_mut};
}

// Override the `init` routine to NOT initialize the TIM7 timer
#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    f3::led::init();
    f3::itm::init();
}
