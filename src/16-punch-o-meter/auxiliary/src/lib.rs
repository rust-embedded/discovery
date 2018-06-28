//! Initialization code

#![feature(panic_implementation)]
#![no_std]

#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate f3;

use core::panic::PanicInfo;

pub use cortex_m::asm::bkpt;
pub use cortex_m::peripheral::ITM;
use cortex_m_rt::ExceptionFrame;
pub use f3::hal::delay::Delay;
use f3::hal::i2c::I2c;
pub use f3::hal::prelude;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
pub use f3::hal::time::MonoTimer;
pub use f3::lsm303dlhc::{I16x3, Sensitivity};
pub use f3::Lsm303dlhc;

pub fn init() -> (Lsm303dlhc, Delay, MonoTimer, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    nss.set_high();

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    let lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    let delay = Delay::new(cp.SYST, clocks);
    let mono_timer = MonoTimer::new(cp.DWT, clocks);

    (lsm303dlhc, delay, mono_timer, cp.ITM)
}

#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    let itm = unsafe { &mut *ITM::ptr() };

    iprintln!(&mut itm.stim[0], "{}", info);

    bkpt();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    bkpt();

    loop {}
}

exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    loop {}
}
