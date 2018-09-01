//! Initialization code

#![feature(panic_implementation)]
#![no_std]

#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate f3;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use cortex_m::asm;
pub use cortex_m::asm::bkpt;
pub use cortex_m::peripheral::ITM;
use cortex_m_rt::ExceptionFrame;

pub fn init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();

    p.ITM
}

#[allow(deprecated)]
#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    let itm = unsafe { &mut *ITM::ptr() };

    iprintln!(&mut itm.stim[0], "{}", info);

    cortex_m::asm::bkpt();

    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {
        // add some side effect to prevent LLVM from turning this into a UDF (abort) instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
