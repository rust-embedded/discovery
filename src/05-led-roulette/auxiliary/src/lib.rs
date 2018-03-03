//! Initialization code

#![no_std]

extern crate cortex_m;
extern crate f3;

pub use f3::hal::delay::Delay;
pub use f3::hal::prelude;
pub use f3::led::Leds;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;

pub fn init() -> (Delay, Leds) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);

    let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    (delay, leds)
}
