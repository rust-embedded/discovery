<!-- # The solution -->

# 解答例

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
    let (i2c1, _delay, mut itm) = aux14::init();

    // ステージ1：読みたいレジスタのアドレスを磁力計に送信します。
    {
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

        // 読みたい`IRA_REG_M`レジスタのアドレスを送信します
        i2c1.txdr.write(|w| w.txdata().bits(IRA_REG_M));

        // 前のバイトが送信されるまで待ちます
        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    // ステージ2：要求したレジスタの内容を受信します
    let byte = {
        // RESTARTをブロードキャストします
        // 磁力計のアドレスをR/WビットをReadに設定して、ブロードキャストします
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(1);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });

        // レジスタの内容を受信するまで待ちます
        while i2c1.isr.read().rxne().bit_is_clear() {}

        // STOPをブロードキャストします（`AUTOEND = 1`なので自動です）

        i2c1.rxdr.read().rxdata().bits()
    };

    // 期待する出力：0x0A - 0b01001000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", IRA_REG_M, byte);

    loop {}
}
```
