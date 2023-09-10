# 读取多个寄存器

读取`IRA_REG_M`寄存器很好地测试了我们对I2C协议的理解，但该寄存器包含了不感兴趣的信息。

这一次，我们将读取实际暴露传感器读数的磁力计寄存器。涉及六个连续寄存器，它们从地址`0x03`处的`OUT_X_H_M`开始。

我们将修改以前的程序以读取这六个寄存器。只需要进行一些修改。

我们需要将磁力计请求的地址从`IRA_REG_M`更改为`OUT_X_H_M`。

``` rust
    // Send the address of the register that we want to read: OUT_X_H_M
    i2c1.txdr.write(|w| w.txdata().bits(OUT_X_H_M));
```

我们必须向从属服务器请求六个字节而不是一个字节。

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

并填充缓冲区，而不是仅读取一个字节：

``` rust
    let mut buffer = [0u8; 6];
    for byte in &mut buffer {
        // Wait until we have received the contents of the register
        while i2c1.isr.read().rxne().bit_is_clear() {}

        *byte = i2c1.rxdr.read().rxdata().bits();
    }

    // Broadcast STOP (automatic because of `AUTOEND = 1`)
```

将所有这些放在一个循环中，并加上一个延迟，以降低数据吞吐量：

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

如果运行此命令，应该在`itmdump`的控制台中每秒打印一个新的六字节数组。
如果在板上移动，数组中的值应该会改变。

``` console
$ # itmdump terminal
(..)
[0, 45, 255, 251, 0, 193]
[0, 44, 255, 249, 0, 193]
[0, 49, 255, 250, 0, 195]
```

但这些字节并没有什么意义。让我们将它们转化为实际读数：

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

现在看起来应该更好了：

``` console
$ # `itmdump terminal
(..)
(44, 196, -7)
(45, 195, -6)
(46, 196, -9)
```

这是沿着磁力仪的XYZ轴分解的地球磁场。

在下一节中，我们将学习如何理解这些数字。
