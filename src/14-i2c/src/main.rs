#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use pg::peripheral;

// Slave address
const MAGNETOMETER: u8 = 0b001_1110;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let i2c1 = unsafe { peripheral::i2c1_mut() };

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    {
        // TODO Broadcast START

        // TODO Broadcast the MAGNETOMETER address with the R/W bit set to Write

        // TODO Send the address of the register that we want to read: IRA_REG_M
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        // TODO Broadcast RESTART

        // TODO Broadcast the MAGNETOMETER address with the R/W bit set to Read

        // TODO Receive the contents of the register

        // TODO Broadcast STOP
        0
    };

    // Expected output: 0x0A - 0b01001000
    iprintln!("0x{:02X} - 0b{:08b}", IRA_REG_M, byte);

    loop {}
}
