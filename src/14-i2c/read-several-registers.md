# Read several registers

Reading the `IRA_REG_M` register was a good test of our understanding of the I2C
protocol but that register contains uninteresting information.

This time, we'll read the registers of the magnetometer that actually expose the
sensor readings. Six contiguous registers are involved and they start with
`OUT_X_H_M` at address `0x03`.

We'll modify our previous program to read these six registers. Only a few
modifications are needed.

We'll need to change the address we request from the magnetometer from
`IRA_REG_M` to `OUT_X_H_M`.

```
// Send the address of the register that we want to read: OUT_X_H_M
i2c1.txdr.write(|w| w.txdata(OUT_X_H_M));
```

We'll have to request the slave for six bytes rather than just one.

``` rust
// Broadcast RESTART
// Broadcast the MAGNETOMETER address with the R/W bit set to Read
i2c1.cr2.modify(|_, w| w.start(true).nbytes(6).rd_wrn(true).autoend(true));
```

And fill a buffer rather than read just one byte:

``` rust
let mut buffer = [0u8; 6];
for byte in &mut buffer {
    // Wait until we have received the contents of the register
    while !i2c1.isr.read().rxne() {}

    *byte = i2c1.rxdr.read().rxdata();
}
// Broadcast STOP (automatic because of `AUTOEND = 1`)
```

Putting it all together inside a loop alongside a delay to reduce the data
throughput:

``` rust
#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let i2c1 = unsafe { peripheral::i2c1_mut() };

    loop {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        i2c1.cr2.write(|w| {
            w.start(true)
                .sadd1(MAGNETOMETER)
                .rd_wrn(false)
                .nbytes(1)
                .autoend(false)
        });

        // Wait until we can send more data
        while !i2c1.isr.read().txis() {}

        // Send the address of the register that we want to read: IRA_REG_M
        i2c1.txdr.write(|w| w.txdata(OUT_X_H_M));

        // Wait until the previous byte has been transmitted
        while !i2c1.isr.read().tc() {}

        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2
            .modify(|_, w| w.start(true).nbytes(6).rd_wrn(true).autoend(true));

        let mut buffer = [0u8; 6];
        for byte in &mut buffer {
            // Wait until we have received something
            while !i2c1.isr.read().rxne() {}

            *byte = i2c1.rxdr.read().rxdata();
        }
        // Broadcast STOP (automatic because of `AUTOEND = 1`)

        iprintln!("{:?}", buffer);

        delay::ms(1_000);
    }
}
```

If you run this, you should printed in the `itmdump`'s console a new array of
six bytes every second. The values within the array should change if you move
around the board.

```
# `itmdump`'s console
[0, 135, 255, 215, 0, 203]
[0, 140, 255, 213, 0, 198]
[0, 138, 255, 215, 0, 201]
[0, 136, 255, 215, 0, 202]
[0, 168, 255, 215, 0, 179]
[1, 39, 255, 223, 0, 73]
[1, 106, 255, 213, 255, 196]
[1, 123, 255, 186, 255, 74]
```

But these bytes don't make much sense like that. Let's turn them into actual
readings:

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

iprintln!("{:?}", (x, y, z));
```

Now it should look better:

```
(363, -179, -111)
(362, -182, -111)
(363, -181, -110)
(356, -163, -99)
(342, -12, -47)
(302, 77, -29)
(197, 176, -37)
(77, 218, -44)
(-25, 220, -53)
```

This is the Earth's magnetic field decomposed alongside the XYZ axis of the
magnetometer.

In the next section, we'll learn how to make sense of these numbers.
