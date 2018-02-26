//! Initialization code

#![feature(lang_items)]
#![feature(macro_reexport)]
#![no_std]

#[macro_use(iprint, iprintln)]
#[macro_reexport(iprint, iprintln)]
extern crate cortex_m;
extern crate f3;

pub use cortex_m::asm::bkpt;
pub use f3::hal::delay::Delay;
pub use f3::hal::prelude;
pub use f3::hal::stm32f30x::i2c1;
use cortex_m::itm;
use cortex_m::peripheral::ITM;
use f3::Lsm303dlhc;
use f3::hal::i2c::I2c;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{self, I2C1};

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
