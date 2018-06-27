#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate aux8;
#[macro_use]
extern crate cortex_m_rt;

entry!(main);

fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // TODO initialize GPIOE

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    aux8::bkpt();

    loop {}
}
