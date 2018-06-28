//! Initialization code

#![feature(panic_implementation)]
#![no_std]

#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate f3;

use core::panic::PanicInfo;

use cortex_m::asm;
pub use cortex_m::asm::bkpt;
use cortex_m::peripheral::ITM;
use cortex_m_rt::ExceptionFrame;
pub use f3::hal::delay::Delay;
use f3::hal::i2c::I2c;
pub use f3::hal::prelude;
use f3::hal::prelude::*;
pub use f3::hal::stm32f30x::i2c1;
use f3::hal::stm32f30x::{self, I2C1};
use f3::Lsm303dlhc;

pub fn init() -> (&'static i2c1::RegisterBlock, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    Lsm303dlhc::new(i2c).unwrap();

    let delay = Delay::new(cp.SYST, clocks);

    unsafe { (&mut *(I2C1::ptr() as *mut _), delay, cp.ITM) }
}

#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    let itm = unsafe { &mut *ITM::ptr() };

    iprintln!(&mut itm.stim[0], "{}", info);

    cortex_m::asm::bkpt();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {}
}

exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    loop {}
}
