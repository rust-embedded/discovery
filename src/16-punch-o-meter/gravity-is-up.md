<!-- # Gravity is up? -->

# 重力は上を向いている？

<!-- What's the first thing we'll do? -->

まず最初にやることはなんでしょうか？

<!-- Perform a sanity check! -->

正当性を検証します！

<!-- 
The starter code prints the X, Y and Z components of the acceleration measured by the accelerometer.
The values have already been "scaled" and have units of `g`s. Where `1 g` is equal to the
acceleration of the gravity, about `9.8` meters per second squared.
 -->

スターターコードは、加速度計で計測した加速度のX、Y、Z要素を表示します。
値はすでに「スケール」されており、その単位は`g`です。ここで、`1 g`は重力加速度と等しく、約`9.8`メートル毎秒毎秒です。

``` rust
{{#include src/main.rs}}
```

<!-- The output of this program with the board sitting still will be something like: -->

ボードが静止している状態で、このプログラムの出力は、次のようになります。

``` console
$ # itmdumpコンソール
(..)
(0.0, 0.0, 1.078125)
(0.0, 0.0, 1.078125)
(0.0, 0.0, 1.171875)
(0.0, 0.0, 1.03125)
(0.0, 0.0, 1.078125)
```

<!-- 
Which is weird because the board is not moving yet its acceleration is non-zero. What's going on?
This must be related to the gravity, right? Because the acceleration of gravity is `1 g`. But the
gravity pulls objects downwards so the acceleration along the Z axis should be negative not positive
...
 -->

これは変です。ボードは動いていないのに、加速度がゼロではありません。何が起こっているのでしょう？
これは、重力と関係しているに違いありません。重力加速度は`1 g`だからです。
しかし、重力は物体を下に引っ張ります。そのため、Z軸の加速度は、正数ではなく負数でなければなりません・・・。

<!-- 
Did the program get the Z axis backwards? Nope, you can test rotating the board to align the gravity
to the X or Y axis but the acceleration measured by the accelerometer is always pointing up.
 -->

このプログラムはZ軸を逆方向に取得しているのでしょうか？いいえ、ボードを回転させて、重力をX軸もしくはY軸に合わせることができますが、
加速度センサで測定された加速度は常に上を向いています。

<!-- 
What happens here is that the accelerometer is measuring the *proper acceleration* of the board not
the acceleration *you* are observing. This proper acceleration is the acceleration of the board as
seen from a observer that's in free fall. An observer that's in free fall is moving toward the
center of the the Earth with an acceleration of `1g`; from its point of view the board is actually
moving upwards (away from the center of the Earth) with an acceleration of `1g`. And that's why the
proper acceleration is pointing up. This also means that if the board was in free fall, the
accelerometer would report a proper acceleration of zero. Please, don't try that at home.
 -->

<!-- `proper acceleration`の意味がわからないので、そのままにしてあります -->

発生している現象は、加速度計は*あなた*が観測している加速度ではなく、ボードの*proper acceleration*を測定している、ということです。
このproper accelerationとは、自由落下中の観測者から見たボードの加速度です。自由落下中の観測者は、地球の中心部に向かって、`1g`の加速度で移動しています。
この観点から見ると、ボードは上に向かって（地球の中心部から離れて）、`1g`の加速度で動いています。
そして、これがproper accelerationが上に向いている理由です。
これはボードが自由落下していると、加速度計はゼロのproper accelerationを報告する、ということを意味します。
家では試さないようにして下さい。

<!-- Yes, physics is hard. Let's move on. -->

物理は難しいです。次へ進みましょう。
