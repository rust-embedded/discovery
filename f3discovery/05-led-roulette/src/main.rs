#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::entry;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
