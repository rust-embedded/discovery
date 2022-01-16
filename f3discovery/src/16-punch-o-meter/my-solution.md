<!-- # My solution -->

# 解答例

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux16::{entry, iprint, iprintln, prelude::*, I16x3, Sensitivity};
use m::Float;

#[entry]
fn main() -> ! {
    const SENSITIVITY: f32 = 12. / (1 << 14) as f32;
    const THRESHOLD: f32 = 0.5;

    let (mut lsm303dlhc, mut delay, mono_timer, mut itm) = aux16::init();

    lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();

    let measurement_time = mono_timer.frequency().0; // 1秒ティック
    let mut instant = None;
    let mut max_g = 0.;
    loop {
        let g_x = f32::from(lsm303dlhc.accel().unwrap().x).abs() * SENSITIVITY;

        match instant {
            None => {
                // 加速度がしきい値を超えると、計測を開始します
                if g_x > THRESHOLD {
                    iprintln!(&mut itm.stim[0], "START!");

                    max_g = g_x;
                    instant = Some(mono_timer.now());
                }
            }
            // まだ計測しています
            Some(ref instant) if instant.elapsed() < measurement_time => {
                if g_x > max_g {
                    max_g = g_x;
                }
            }
            _ => {
                // 最大値を報告します
                iprintln!(&mut itm.stim[0], "Max acceleration: {}g", max_g);

                // 計測を終了します
                instant = None;

                // リセットします
                max_g = 0.;
            }
        }

        delay.delay_ms(50_u8);
    }
}
```
