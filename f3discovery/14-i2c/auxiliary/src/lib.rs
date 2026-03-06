//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] //  bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln};
pub use cortex_m_rt::entry;
pub use stm32f3_discovery::stm32f3xx_hal::{delay::Delay, prelude, stm32::i2c1};

use cortex_m::peripheral::ITM;
use stm32f3_discovery::{
    lsm303dlhc::Lsm303dlhc,
    stm32f3xx_hal::{
        i2c::I2c,
        prelude::*,
        stm32::{self, I2C1},
    },
};

pub fn init() -> (&'static i2c1::RegisterBlock, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    Lsm303dlhc::new(i2c).unwrap();

    let delay = Delay::new(cp.SYST, clocks);

    unsafe { (&mut *(I2C1::ptr() as *mut _), delay, cp.ITM) }
}
