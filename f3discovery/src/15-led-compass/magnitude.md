<!-- # Magnitude -->

# 大きさ

<!-- 
We have been working with the direction of the magnetic field but what's its real magnitude? The
number that the `magnetic_field` function reports are unit-less. How can we convert those values to
Gauss?
 -->

磁場の方向を取り扱ってきましたが、実際の大きさはどうなのでしょうか？
`magnetic_field`関数が報告する数字は、単位がありません。どのようにしてこの値をガウスに変換するのでしょうか？

<!-- The documentation will answer that question. -->

ドキュメントがその質問に答えます。

> Section 2.1 Sensor characteristics - Page 10 - LSM303DLHC Data Sheet

<!-- 
The table in that page shows a *magnetic gain setting* that has different values according to the
values of the GN bits. By default, those GN bits are set to `001`. That means that magnetic gain of
the X and Y axes is `1100 LSB / Gauss` and the magnetic gain of the Z axis is `980 LSB / Gauss`. LSB
stands for Least Significant Bits and the `1100 LSB / Gauss` number indicates that a reading of
`1100` is equivalent to `1 Gauss`, a reading of `2200` is equivalent to 2 Gauss and so on.
 -->

上記ページの表は、GNビットの値に応じて、異なる値を取る*磁気のゲイン設定*を説明しています。
デフォルトでは、GNビットは`001`に設定されています。これは、XとY軸の磁気ゲインは、`1100 LSB / Gauss`であることを意味います。
そして、Z軸の磁気ゲインは、`980 LSB / Gauss`です。
LSBは、Least Significant Bitsを意味しており、`1100 LSB / Gauss`は、`1100`が`1 Gauss`と等価であり、
`2200`は`2 Gauss`と等価である、ということを意味しています。

<!-- 
So, what we need to do is divide the X, Y and Z values that the sensor outputs by its corresponding
*gain*. Then, we'll have the X, Y and Z components of the magnetic field in Gauss.
 -->

つまり、センサ出力のX、Y、Zの値を、対応する*ゲイン*で除算することが必要です。
すると、X、Y、Z要素の磁場のガウスが計算できます。

<!-- 
With some extra math we can retrieve the magnitude of the magnetic field from its X, Y and Z
components:
 -->

いくつか追加で計算することで、X、Y、Zの要素から磁場の大きさを取得できます。

``` rust
let magnitude = (x * x + y * y + z * z).sqrt();
```

<!-- Putting all this together in a program: -->

これらをプログラムにまとめます。

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, I16x3};
use m::Float;

#[entry]
fn main() -> ! {
    const XY_GAIN: f32 = 1100.; // LSB / G
    const Z_GAIN: f32 = 980.; // LSB / G

    let (_leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();

    loop {
        let I16x3 { x, y, z } = lsm303dlhc.mag().unwrap();

        let x = f32::from(x) / XY_GAIN;
        let y = f32::from(y) / XY_GAIN;
        let z = f32::from(z) / Z_GAIN;

        let mag = (x * x + y * y + z * z).sqrt();

        iprintln!(&mut itm.stim[0], "{} mG", mag * 1_000.);

        delay.delay_ms(500_u16);
    }
}
```

<!-- 
This program will report the magnitude (strength) of the magnetic field in milligauss (`mG`). The
magnitude of the Earth's magnetic field is in the range of `250 mG` to `650 mG` (the magnitude
varies depending on your geographical location) so you should see a value in that range or close to
that range -- I see a magnitude of around 210 mG.
 -->

このプログラムは、磁場の大きさ（強さ）をミリガウス（`mG`）単位で報告します。
地球磁場の大きさは、`250 mG`から`650 mG`の範囲になります（この大きさは、地理的な場所によって変化します）。
そのため、この範囲内か、範囲に近い値が観測できるはずです。私は、約210 mGの大きさを観測しました。

<!-- Some questions: -->

いくつかの質問：

<!-- Without moving the board, what value do you see? Do you always see the same value? -->

ボードを動かさずに、どのような値を観測しましたか？常に同じ値ですか？

<!-- If you rotate the board, does the magnitude change? Should it change? -->

ボードを回転すると、大きさは変化しますか？変化すべきですか？
