<!-- # Putting it all together -->

# 全てをまとめる

``` rust
#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // `ms`ティック後にオフになるようにタイマを設定します。
    // 1ティックは1msです。
    tim6.arr.write(|w| w.arr().bits(ms));

    // CEN：カウンタを有効化します。
    tim6.cr1.modify(|_, w| w.cen().set_bit());

    // アラームがオフになるまで（更新イベントが発生するまで）待ちます
    while !tim6.sr.read().uif().bit_is_set() {}

    // 更新イベントフラグをクリアします
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // TIM6のタイマの電源を入れます。
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());

    // OPM：ワンパルスモードを選択します。
    // CEN：今はカウンタを無効にしておきます。
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

    // カウンタが1KHzで動作するようにプリスケーラを設定します。
    // APB1_CLOCK = 8 MHz
    // PSC = 7999
    // 8 MHz / (7999 + 1) = 1 KHz
    // カウンタ（CNT）は、毎ミリ秒ごとに増加します。
    tim6.psc.write(|w| w.psc().bits(7_999));

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
```
