#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        // 魔法のアドレス！
        const GPIOE_BSRR: u32 = 0x48001018;

        // 「北」のLED（赤）を点灯します
        *(GPIOE_BSRR as *mut u32) = 1 << 9;

        // 「東」のLED（緑）を点灯します
        *(GPIOE_BSRR as *mut u32) = 1 << 11;

        // 「北」のLEDを消灯します
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

        // 「東」のLEDを消灯します
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
    }

    loop {}
}
