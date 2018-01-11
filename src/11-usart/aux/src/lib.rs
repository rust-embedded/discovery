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
pub use f3::hal::prelude;
pub use f3::hal::serial::Serial;
pub use f3::hal::stm32f30x::usart1;
pub use f3::hal::time::MonoTimer;
use cortex_m::itm;
use f3::hal::prelude::*;
use f3::hal::stm32f30x::{self, USART1};

pub fn init() -> (&'static mut usart1::RegisterBlock, MonoTimer, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    Serial::usart1(dp.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);

    unsafe {
        (
            &mut *(USART1::ptr() as *mut _),
            MonoTimer::new(cp.DWT, clocks),
            cp.ITM,
        )
    }
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
