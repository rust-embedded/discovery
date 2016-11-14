//! Playground

#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(bkpt, iprint, iprintln)]
extern crate f3;

/// Low level access to peripherals
pub mod peripheral {
    pub use f3::peripheral::{rcc, rcc_mut, gpioe, gpioe_mut};
}

// Override the `init` routine to NOT initialize the GPIOE port
#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    f3::itm::init();
}
