<!-- # Take 1 -->

# 第一弾

<!-- What's the simplest way in which we can implement the LED compass? Even if it's not perfect. -->

LEDをコンパスを実装する最も単純な方法はどのようなものでしょうか？例えそれが完璧でなかったとしても。

<!-- 
For starters, we'd only care about the X and Y components of the magnetic field because when you
look at a compass you always hold it in horizontal position thus the compass is in the XY plane.
 -->

初心者のために、磁場のXとYの要素だけを取り扱います。なぜならコンパスを見る時、常にコンパスを水平に保ちます。
従って、コンパスはXY平面にあると考えます。

<!-- 
For example, what LED would you turn on in the following case. EMF stands for Earth's Magnetic Field
and green arrow has the direction of the EMF (it points north).
 -->

例えば、次の場合に、どのLEDを点灯するでしょうか。EMFは地球磁場を意味しており、緑の矢印はEMFの方向です（北を向いています）。

<p align="center">
<img title="Quadrant I" src="../assets/quadrant-i.png">
</p

<!-- The `Southeast` LED, right? -->

`南東`のLEDで良いでしょうか？

<!-- 
What *signs* do the X and Y components of the magnetic field have in that scenario? Both are
positive.
 -->

この場合、磁場のXとY要素の*符号*は何になっているでしょうか？両方ともに正数です。

<!-- 
If we only looked at the signs of the X and Y components we could determine to which quadrant the
magnetic field belongs to.
 -->

XとY要素の符号だけを見る場合、磁場が4象限のどこに属しているか、を決定できます。

<p align="center">
<img class="white_bg" title="Quadrants" src="../assets/quadrants.png">
</p>

<!-- 
In the previous example, the magnetic field was in the first quadrant (x and y were positive) and it
made sense to turn on the `SouthEast` LED. Similarly, we could turn a different LED if the magnetic
field was in a different quadrant.
 -->

前の例では、磁場は第一象限に属していました（xとyは正）。そして、`南東`のLEDを点灯することがわかりました。
同様に、磁場が別の象限に属している場合に、別のLEDを点灯します。

<!-- Let's try that logic. Here's the starter code: -->

このロジックを試してみましょう。スターターコードは、次の通りです。

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, switch_hal::OutputSwitch, Direction, I16x3};

#[entry]
fn main() -> ! {
    let (leds, mut lsm303dlhc, mut delay, _itm) = aux15::init();
    let mut leds = leds.into_array();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        // 磁場がどの象限に属するか決めるため、XとY要素の符号を見ます。
        let dir = match (x > 0, y > 0) {
            // 第???象限
            (true, true) => Direction::Southeast,
            // 第???象限
            (false, true) => panic!("TODO"),
            // 第???象限
            (false, false) => panic!("TODO"),
            // 第???象限
            (true, false) => panic!("TODO"),
        };

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(1_000_u16);
    }
}
```

<!-- 
There's a `Direction` enum in the `led` module that has 8 variants named after the cardinal points:
`North`, `East`, `Southwest`, etc. Each of these variants represent one of the 8 LEDs in the
compass. The `Leds` value can be indexed using the `Direction` `enum`; the result of indexing is the
LED that points in that `Direction`.
 -->

`led`モジュールには、`Direction`列挙体があります。
この列挙体は、`North`、`East`、`Southwest`のように、方位に基づいて名付けられた8個のヴァリアントがあります。
各ヴァリアントは、円形に並んだ8個のLEDを表しています。
`Leds`の値は、`Direction` `enum`を使ってインデックスアクセスできます。
インデックスアクセスの結果は、`Direction`を向いたLEDです。
