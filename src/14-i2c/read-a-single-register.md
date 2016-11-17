# Read a single register

Let's put all that theory into practice!

Just like with the USART peripheral, I've took care of initializing everything
before you reach `main` so you'll only have to deal with the following
registers:

- `CR2`. Control register 2.
- `ISR`. Interrupt and status register.
- `TXDR`. Transmit data register.
- `RXDR`. Receive data register.

These registers are documented in the following section of the Reference Manual:

> Section 28.7 I2C registers - Page 873 - Reference Manual

We'll be using the `I2C1` peripheral in conjunction with pins `PB6` (`SCL`) and
`PB7` (`SDA`).

You won't have to wire anything this time because the sensor is on the board and
it's already connected to the microcontroller. However, I do would recommend
that you disconnect the serial / Bluetooth module from the F3 to make it
easier to manipulate. Later on, we'll be moving the board around quite a bit.

Your task is to write a program that reads the contents of the magnetometer's
`IRA_REG_M` register. This register is read only and always contains the value
`0b01001000`.

The microcontroller will be taking the role of the I2C master and the
magnetometer inside the LSM303DLHC will be the I2C slave.

Here's the starter code. You'll have to implement the `TODO`s.

``` rust
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
    };

    // Expected output: 0x0A - 0b01001000
    iprintln!("0x{:02X} - 0b{:08b}", IRA_REG_M, byte);

    loop {}
}
```

To give you some extra help, these are the exact bits you'll be working with:

- `CR2`: `SADD1`, `RD_WRN`, `NBYTES`, `START`, `AUTOEND`
- `ISR`: `TXIS`, `RXNE`, `TC`
- `TXDR`: `TXDATA`
- `RXDR`: `RXDATA`
