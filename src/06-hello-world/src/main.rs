#![deny(unsafe_code)]
#![no_std]

#[macro_use]
extern crate aux6;

fn main() {
    let mut itm = aux6::init();

    iprintln!(&mut itm.stim[0], "Hello, world!");
}
