<!-- # One-shot timer -->

# ワンショットタイマ

<!-- I hope that, by now, I have convinced you that `for` loop delays are a poor way to implement delays. -->

これまでに、`for`ループでの遅延は、遅延を実装する方法としては良くない方法であると納得してもらっていると思います。

<!-- 
Now, we'll implement delays using a *hardware timer*. The basic function of a (hardware) timer is
... to keep precise track of time. A timer is yet another peripheral that's available to the
microcontroller; thus it can be controlled using registers.
 -->

ここでは、*ハードウェアタイマ*を使って遅延を実装します。（ハードウェア）タイマの基本的な機能は、時間を正確に追跡することです。
タイマは、マイクロコントローラから利用できるさらに別のペリフェラルです。
そのため、レジスタを使って制御できます。

<!-- 
The microcontroller we are using has several (in fact, more than 10) timers of different kinds
(basic, general purpose, and advanced timers) available to it. Some timers have more *resolution*
(number of bits) than others and some can be used for more than just keeping track of time.
 -->

私たちが利用しているマイクロコントローラは、いくつかの（実は10を超える数の）異なる種類（簡易、汎用、高度なタイマ）のタイマを持っています。
いくつかのタイマは、他のタイマより高い分解能（ビット数）を持ちます。そして、単純に時間を追跡すること以上の用途で使えるものもあります。

<!-- 
We'll be using one of the *basic* timers: `TIM6`. This is one of the simplest timers available in
our microcontroller. The documentation for basic timers is in the following section:
 -->

`TIM6`という*簡易*タイマの1つを利用します。このもっとも単純なタイマは、マイクロコントローラ内で利用可能です。
この簡易タイマのドキュメントは、下記にあります。

> Section 22 Timers - Page 670 - Reference Manual

<!-- Its registers are documented in: -->

レジスタに関する記述は、下記にあります。

> Section 22.4.9 TIM6/TIM7 register map - Page 682 - Reference Manual

<!-- The registers we'll be using in this section are: -->

このセクションで利用するレジスタは、下記の通りです。

<!-- 
- `SR`, the status register.
- `EGR`, the event generation register.
- `CNT`, the counter register.
- `PSC`, the prescaler register.
- `ARR`, the autoreload register.
 -->

- `SR`、ステータスレジスタ。
- `EGT`、イベント生成レジスタ。
- `CNT`、カウンタレジスタ。
- `PSC`、プリスケーラレジスタ。
- `ARR`、自動リロードレジスタ。

<!-- 
We'll be using the timer as a *one-shot* timer. It will sort of work like an alarm clock. We'll set
the timer to go off after some amount of time and then we'll wait until the timer goes off. The
documentation refers to this mode of operation as *one pulse mode*.
 -->

タイマを*ワンショット*タイマとして使用します。これは、目覚まし時計のような役割を果たします。
ある程度時間が経過してからタイマがオフになるように設定してから、タイマがオフになるまで待ちます。
ドキュメント内では、この動作モードを*ワンパルスモード*と呼んでいます。

<!-- 
Here's a description of how a basic timer works when configured in one pulse mode:
 -->

ここに、簡易タイマをワンパルスモードとして設定する方法を記載します。

<!-- 
- The counter is enabled by the user (`CR1.CEN = 1`).
- The `CNT` register resets its value to zero and, on each tick, its value gets incremented by one.
- Once the `CNT` register has reached the value of the `ARR` register, the counter will be disabled
  by hardware (`CR1.CEN = 0`) and an *update event* will be raised (`SR.UIF = 1`).
 -->

- カウンタをユーザーによって有効化します（`CR1.CEN = 1`）。
- `CNT`レジスタの値をゼロにリセットします。この値は、ティックごとに値が1つずつインクリメントされます。
- 一度`CNT`レジスタが`ARR`レジスタの値に到達すると、カウンタがハードウェアによって無効になります（`CR1.CEN = 0`）。
  そして、*更新イベント*（`SR.UIF = 1`）が通知されます。

<!-- 
`TIM6` is driven by the APB1 clock, whose frequency doesn't have to necessarily match the processor
frequency. That is, the APB1 clock could be running faster or slower. The default, however, is that
both APB1 and the processor are clocked at 8 MHz.
 -->

`TIM6`はAPB1クロックによって駆動されます。APB1のクロック周波数は、プロセッサの周波数と一致している必要はありません。
APB1クロックは、プロセッサより速かったり遅かったりします。しかし、デフォルトでは、APB1とプロセッサのクロックは両方とも8MHzです。

<!-- 
The tick mentioned in the functional description of the one pulse mode is *not* the same as one
tick of the APB1 clock. The `CNT` register increases at a frequency of `apb1 / (psc + 1)`
times per second, where `apb1` is the frequency of the APB1 clock and `psc` is the value of the
prescaler register, `PSC`.
 -->

ワンパルスモードの機能説明で書かれているティックは、APB1クロックの1ティックと同じ*ではありません*。
`CNT`レジスタは、毎秒`apb1 / (psc + 1)`の周波数で増加します。
ここで、`apb1`はAPB1クロックの周波数で、`psc`はプリスケーラレジスタ（`PSC`）の値です。
