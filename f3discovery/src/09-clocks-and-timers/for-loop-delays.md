<!-- # `for` loop delays -->

# `for`ループで遅延

<!-- 
The first challenge is to implement the `delay` function without using any peripheral and the
obvious solution is to implement it as a `for` loop delay:
 -->

最初の課題は、`delay`関数をペリフェラルを使わずに実装することです。
明らかな解決策は、`for`ループで遅延を実装することです。

``` rust
#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    for _ in 0..1_000 {}
}
```

<!-- 
Of course, the above implementation is wrong because it always generates the same delay for any
value of `ms`.
 -->

もちろん、上の実装は間違っています。どんな`ms`の値に対しても、常に同じ遅延を生成するからです。

<!-- In this section, you'll have to: -->

このセクションでは、次のことに取り組む必要があります。

<!-- 
- Fix the `delay` function to generate delays proportional to its input `ms`.
- Tweak the `delay` function to make the LED roulette spin at a rate of approximately 5 cycles in 4
  seconds (800 milliseconds period).
- The processor inside the microcontroller is clocked at 72 MHz and executes most instructions in one
  "tick", a cycle of its clock. How many (`for`) loops do  you *think* the `delay` function must do
  to generate a delay of 1 second?
- How many `for` loops does `delay(1000)` actually do?
- What happens if compile your program in release mode and run it?
 -->

- 入力の`ms`に比例した遅延を生成するように、`delay`関数を修正します。
- LEDルーレットが4秒に5回（1周期800ms）程度回るように、`delay`関数を微調整します。
- マイクロコントローラ内のプロセッサは、8MHzのクロックで駆動されており、ほとんどの命令を1「ティック」（1クロックサイクル）で実行します。
  1秒の遅延を生成するために、何回の（`for`）ループが、`delay`関数に必要だと考えますか？
- `delay(1000)`が実際に実行する`for`ループは何回でしょうか？
- プログラムをリリースモードでコンパイルし、実行すると、何が起こりますか？
