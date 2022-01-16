#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// スレーブアドレス
const MAGNETOMETER: u16 = 0b0011_1100;

// 磁力計レジスタのアドレス
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[entry]
fn main() -> ! {
    let (i2c1, _delay, mut itm) = aux14::init();

    // ステージ1：読みたいレジスタのアドレスを磁力計に送信します。
    {
        // TODO STARTをブロードキャストします

        // TODO 磁力計のアドレスをR/WビットをWriteに設定して、ブロードキャストします

        // TODO 読みたい`IRA_REG_M`レジスタのアドレスを送信します
    }

    // ステージ2：要求したレジスタの内容を受信します
    let byte = {
        // TODO RESTARTをブロードキャストします

        // TODO 磁力計のアドレスをR/WビットをReadに設定して、ブロードキャストします

        // TODO レジスタの内容を受信します

        // TODO STOPをブロードキャストします
        0
    };

    // 期待する出力：0x0A - 0b01001000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", IRA_REG_M, byte);

    loop {}
}
