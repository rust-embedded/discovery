# The solution

``` rust
#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let i2c1 = unsafe { peripheral::i2c1_mut() };

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    {
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
        i2c1.txdr.write(|w| w.txdata(IRA_REG_M));

        // Wait until the previous byte has been transmitted
        while !i2c1.isr.read().tc() {}
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2
            .modify(|_, w| w.start(true).nbytes(1).rd_wrn(true).autoend(true));

        // Wait until we have received the contents of the register
        while !i2c1.isr.read().rxne() {}

        // Broadcast STOP (automatic because of `AUTOEND = 1`)

        i2c1.rxdr.read().rxdata()
    };

    // Expected output: 0x0A - 0b01001000
    iprintln!("0x{:02X} - 0b{:08b}", IRA_REG_M, byte);

    loop {}
}
```
