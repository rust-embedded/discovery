<!-- # Clocks and timers -->

# クロックと時間

<!-- 
In this section, we'll re-implement the LED roulette application. I'm going to give you back the
`Led` abstraction but this time I'm going to take away the `Delay` abstraction `:-)`.
 -->

このセクションでは、LEDルーレットアプリケーションを再実装します。`Led`の抽象化をお返ししますが、
`Delay`の抽象化を取り除いていきます。

<!-- 
Here's the starter code. The `delay` function is unimplemented so if you run this program the LEDs
will blink so fast that they'll appear to always be on.
 -->

スターターコードはこちらです。`delay`関数は、未実装です。プログラムを実行すると、LEDの点滅が早すぎて、常に点灯しているように見えるでしょう。

``` rust
{{#include src/main.rs}}
```
