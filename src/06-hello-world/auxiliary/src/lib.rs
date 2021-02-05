//! Initialization code

#![no_std]

pub use panic_itm;

pub use cortex_m_rt::entry;

// Need stm32f3xx_hal::prelude::* otherwise
//   'Error(corex-m-rt): The interrupt vectors are missing`
pub use stm32f3_discovery::stm32f3xx_hal::prelude::*;

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};

pub fn init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();

    p.ITM
}
