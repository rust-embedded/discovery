<!-- # Read several registers -->

# 複数のレジスタを読む

<!-- 
Reading the `IRA_REG_M` register was a good test of our understanding of the I2C protocol but that
register contains uninteresting information.
 -->

`IRA_REG_M`レジスタを読むことは、I2Cプロトコルの理解を試すには良いものでした。
しかし、レジスタの内容はおもしろみのない情報です。

<!-- 
This time, we'll read the registers of the magnetometer that actually expose the sensor readings.
Six contiguous registers are involved and they start with `OUT_X_H_M` at address `0x03`.
 -->

今回は、実際のセンサの測定値がわかる磁力計のレジスタを読みます。
6つの連続したレジスタが関係しており、そのレジスタは`0x03`番地の`OUT_X_H_M`から始まります。

<!-- 
We'll modify our previous program to read these six registers. Only a few modifications are needed.
 -->

前のプログラムを、これら6つのレジスタを読むように修正します。ほんの少しの修正だけで済みます。

<!-- 
We'll need to change the address we request from the magnetometer from `IRA_REG_M` to `OUT_X_H_M`.
 -->

磁力計に要求するアドレスを、`IRA_REG_M`から`OUT_X_H_M`に変更します。

``` rust
    // 読みたい`OUT_X_H_M`レジスタのアドレスを送信します
    i2c1.txdr.write(|w| w.txdata().bits(OUT_X_H_M));
```

<!-- We'll have to request the slave for six bytes rather than just one. -->

1バイトだけではなく、6バイトを要求します。

``` rust
    // RESTARTをブロードキャストします
    // 磁力計のアドレスをR/WビットをReadに設定して、ブロードキャストします
    i2c1.cr2.modify(|_, w| {
        w.start().set_bit();
        w.nbytes().bits(6);
        w.rd_wrn().set_bit();
        w.autoend().set_bit()
    });
```

<!-- And fill a buffer rather than read just one byte: -->

1バイトだけ読むのではなく、バッファを埋めます。

``` rust
    let mut buffer = [0u8; 6];
    for byte in &mut buffer {
        // レジスタの内容を受信するまで待ちます
        while i2c1.isr.read().rxne().bit_is_clear() {}

        *byte = i2c1.rxdr.read().rxdata().bits();
    }

    // STOPをブロードキャストします（`AUTOEND = 1`なので自動です）
```

<!-- 
Putting it all together inside a loop alongside a delay to reduce the data throughput:
 -->

データのスループットを減らすための遅延と共に、これらをループ内にまとめると、次のようになります。

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// スレーブアドレス
const MAGNETOMETER: u8 = 0b001_1110;

// 磁力計レジスタのアドレス
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm) = aux14::init();

    loop {
        // STARTをブロードキャストします
        // 磁力計のアドレスをR/WビットをWriteに設定して、ブロードキャストします
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });

        // 次のデータを送信できるようになるまで、待ちます
        while i2c1.isr.read().txis().bit_is_clear() {}

        // 読みたい`OUT_X_H_M`レジスタのアドレスを送信します
        i2c1.txdr.write(|w| w.txdata().bits(OUT_X_H_M));

        // 前のバイトが送信されるまで待ちます
        while i2c1.isr.read().tc().bit_is_clear() {}

        // RESTARTをブロードキャストします
        // 磁力計のアドレスをR/WビットをReadに設定して、ブロードキャストします
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(6);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });

        let mut buffer = [0u8; 6];
        for byte in &mut buffer {
            // レジスタの内容を受信するまで待ちます
            while i2c1.isr.read().rxne().bit_is_clear() {}

            *byte = i2c1.rxdr.read().rxdata().bits();
        }
        // STOPをブロードキャストします（`AUTOEND = 1`なので自動です）

        iprintln!(&mut itm.stim[0], "{:?}", buffer);

        delay.delay_ms(1_000_u16);
    }
}
```

<!-- 
If you run this, you should printed in the `itmdump`'s console a new array of six bytes every
second. The values within the array should change if you move around the board.
 -->

これを実行すると、毎秒新しい6バイトの配列が`itmdump`のコンソールに表示されるはずです。
配列内の値は、ボードを動かすと変化するはずです。

``` console
$ # itmdump terminal
(..)
[0, 45, 255, 251, 0, 193]
[0, 44, 255, 249, 0, 193]
[0, 49, 255, 250, 0, 195]
```

<!-- But these bytes don't make much sense like that. Let's turn them into actual readings: -->

しかし、これらのバイト列は、それほど意味がありません。実際の測定値に変換しましょう。

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

<!-- Now it should look better: -->

これで、より見やすくなるはずです。

``` console
$ # `itmdump terminal
(..)
(44, 196, -7)
(45, 195, -6)
(46, 196, -9)
```

<!-- 
This is the Earth's magnetic field decomposed alongside the XYZ axis of the magnetometer.
 -->

これは、磁力計のXYZ軸で分解された地球磁場です。

<!-- In the next section, we'll learn how to make sense of these numbers. -->

次のセクションでは、これらの数字を理解する方法を学びます。