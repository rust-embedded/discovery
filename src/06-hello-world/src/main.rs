#![deny(unsafe_code)]
#![no_std]

#[macro_use]
extern crate aux;

fn main() {
    let mut itm = aux::init();

    iprintln!(&mut itm.stim[0], "Hello, world!");
}
