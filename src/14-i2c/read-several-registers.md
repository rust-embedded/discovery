# Read several registers

Reading the `IRA_REG_M` register was a good test of our understanding of the I2C protocol but that
register contains uninteresting information.

This time, we'll read the registers of the magnetometer that actually expose the sensor readings.
Six contiguous registers are involved and they start with `OUT_X_H_M` at address `0x03`.

We'll modify our previous program to read these six registers. Only a few modifications are needed.

We'll need to change the address we request from the magnetometer from `IRA_REG_M` to `OUT_X_H_M`.

``` rust
    // Send the address of the register that we want to read: OUT_X_H_M
    i2c1.txdr.write(|w| w.txdata().bits(OUT_X_H_M));
```

We'll have to request the slave for six bytes rather than just one.

``` rust
    // Broadcast RESTART
    // Broadcast the MAGNETOMETER address with the R/W bit set to Read
    i2c1.cr2.modify(|_, w| {
        w.start().set_bit();
        w.nbytes().bits(6);
        w.rd_wrn().set_bit();
        w.autoend().set_bit()
    });
```

And fill a buffer rather than read just one byte:

``` rust
    let mut buffer = [0u8; 6];
    for byte in &mut buffer {
        // Wait until we have received the contents of the register
        while i2c1.isr.read().rxne().bit_is_clear() {}

        *byte = i2c1.rxdr.read().rxdata().bits();
    }

    // Broadcast STOP (automatic because of `AUTOEND = 1`)
```

Putting it all together inside a loop alongside a delay to reduce the data throughput:

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm) = aux14::init();

    loop {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });

        // Wait until we can send more data
        while i2c1.isr.read().txis().bit_is_clear() {}

        // Send the address of the register that we want to read: OUT_X_H_M
        i2c1.txdr.write(|w| w.txdata().bits(OUT_X_H_M));

        // Wait until the previous byte has been transmitted
        while i2c1.isr.read().tc().bit_is_clear() {}

        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(6);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });

        let mut buffer = [0u8; 6];
        for byte in &mut buffer {
            // Wait until we have received something
            while i2c1.isr.read().rxne().bit_is_clear() {}

            *byte = i2c1.rxdr.read().rxdata().bits();
        }
        // Broadcast STOP (automatic because of `AUTOEND = 1`)

        iprintln!(&mut itm.stim[0], "{:?}", buffer);

        delay.delay_ms(1_000_u16);
    }
}
```

If you run this, you should printed in the `itmdump`'s console a new array of six bytes every
second. The values within the array should change if you move around the board.

``` console
$ # itmdump terminal
(..)
[0, 45, 255, 251, 0, 193]
[0, 44, 255, 249, 0, 193]
[0, 49, 255, 250, 0, 195]
```

But these bytes don't make much sense like that. Let's turn them into actual readings:

``` rust
        let x_h = u16::from(buffer[0]);
        let x_l = u16::from(buffer[1]);
        let z_h = u16::from(buffer[2]);
        let z_l = u16::from(buffer[3]);
        let y_h = u16::from(buffer[4]);
        let y_l = u16::from(buffer[5]);

        let x = ((x_h << 8) + x_l) as i16;
        let y = ((y_h << 8) + y_l) as i16;
        let z = ((z_h << 8) + z_l) as i16;

        iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));
```

Now it should look better:

``` console
$ # `itmdump terminal
(..)
(44, 196, -7)
(45, 195, -6)
(46, 196, -9)
```

This is the Earth's magnetic field decomposed alongside the XYZ axis of the magnetometer.

In the next section, we'll learn how to make sense of these numbers.
