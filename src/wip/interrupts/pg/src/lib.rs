#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(bkpt, iprint, iprintln)]
extern crate f3;

extern crate cortex_m;

#[doc(hidden)]
pub use f3::itm;

pub use f3::{delay, led};

/// Low level access to peripherals
pub mod peripheral {
    pub use f3::peripheral::{usart1, usart1_mut};
}

#[export_name = "_init"]
pub unsafe extern "C" fn init() {
    f3::itm::init();
    f3::led::init();
    f3::serial::init();

    let rcc = f3::peripheral::rcc_mut();
    let tim7 = f3::peripheral::tim7_mut();

    // RCC: Enable TIM7
    rcc.apb1enr.modify(|_, w| w.tim7en(true));

    // CEN: Disable the clock
    // OPM. Enable continuous mode. The timer will restart when the counter
    // reaches the value of the auto-reload register.
    tim7.cr1.write(|w| w.cen(false).opm(false));

    // Enable interrupts on timer "updates"
    tim7.dier.write(|w| w.uie(true));

    // Set pre-scaler to 8_000 -> Timer frequency = 1 KHz
    tim7.psc.write(|w| w.psc(7_999));

    // Configure the auto-reload register for 50ms interrupts
    tim7.arr.write(|w| w.arr(50));

    // Generate an update event to update the auto-reload register
    tim7.egr.write(|w| w.ug(true));

    // CEN: Enable the counter
    tim7.cr1.modify(|_, w| w.cen(true));

    // "Unmask" the TIM7 interrupt (N = 55)
    let nvic = cortex_m::peripheral::nvic_mut();
    nvic.iser[1].write(1 << (55 - 32));
}
