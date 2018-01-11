//! Initialization code

#![feature(lang_items)]
#![feature(macro_reexport)]
#![no_std]

#[macro_use(iprint, iprintln)]
#[macro_reexport(iprint, iprintln)]
extern crate cortex_m;
extern crate f3;

pub use cortex_m::asm::bkpt;
pub use cortex_m::peripheral::ITM;
pub use f3::Lsm303dlhc;
pub use f3::hal::delay::Delay;
pub use f3::hal::prelude;
pub use f3::hal::time::MonoTimer;
pub use f3::lsm303dlhc::{I16x3, Sensitivity};
use cortex_m::itm;
use f3::hal::i2c::I2c;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;

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

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(
    args: ::core::fmt::Arguments,
    file: &'static str,
    line: u32,
    col: u32,
) -> ! {
    let itm = &mut *ITM::ptr();

    itm::write_str(&mut itm.stim[0], "PANIC at '");
    itm::write_fmt(&mut itm.stim[0], args);
    iprintln!(&mut itm.stim[0], "', {}:{}:{}", file, line, col);

    cortex_m::asm::bkpt();

    loop {}
}
