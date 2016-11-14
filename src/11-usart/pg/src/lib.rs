//! Playground

#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(bkpt, iprint, iprintln)]
extern crate f3;

#[doc(hidden)]
pub use f3::itm;

pub use f3::{led, time};

/// Low level access to peripherals
pub mod peripheral {
    pub use f3::peripheral::{usart1, usart1_mut};
}

#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    f3::fpu::init();
    f3::itm::init();
    f3::led::init();
    f3::serial::init();
    f3::time::init();
}
