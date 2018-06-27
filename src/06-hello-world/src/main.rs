#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate aux6;
#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;

entry!(main);

fn main() -> ! {
    let mut itm = aux6::init();

    iprintln!(&mut itm.stim[0], "Hello, world!");

    loop {}
}
