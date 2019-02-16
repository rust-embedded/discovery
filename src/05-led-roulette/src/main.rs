#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::entry;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    // 無限ループ。このスタックフレームから抜けないためのものです。
    loop {}
}
