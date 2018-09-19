//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] //  bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln};
pub use cortex_m_rt::entry;
pub use f3::hal::{delay::Delay, prelude, stm32f30x::i2c1};

use cortex_m::peripheral::ITM;
use f3::{
    hal::{
        i2c::I2c,
        prelude::*,
        stm32f30x::{self, I2C1},
    },
    Lsm303dlhc,
};

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
