#![no_std]
#![no_main]

extern crate pg;

use pg::peripheral;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let (gpioe, rcc) =
        unsafe { (peripheral::gpioe_mut(), peripheral::rcc_mut()) };

    // TODO initialize GPIOE

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8(true)
            .odr9(true)
            .odr10(true)
            .odr11(true)
            .odr12(true)
            .odr13(true)
            .odr14(true)
            .odr15(true)
    });

    loop {}
}
