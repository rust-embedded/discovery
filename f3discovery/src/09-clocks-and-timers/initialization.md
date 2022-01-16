<!-- # Initialization -->

# 初期化

<!-- 
As with every other peripheral, we'll have to initialize this timer before we can use it. And just
as in the previous section, initialization is going to involve two steps: powering up the timer and
then configuring it.
 -->

他のペリフェラルと同様に、タイマを使う前に、タイマを初期化する必要があります。
前のセクションと同様に、初期化には2つの手順があります。タイマの電源を入れることとタイマを設定することです。

<!-- 
Powering up the timer is easy: We just have to set `TIM6EN` bit to 1. This bit is in the `APB1ENR`
register of the `RCC` register block.
 -->

タイマの電源を入れることは簡単です。`TIM6EN`ビットに1を設定するだけです。
このビットは、`RCC`レジスタブロックの`APB1ENR`レジスタの中にあります。

``` rust
    // TIM6のタイマの電源を入れます。
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());
```

<!-- The configuration part is slightly more elaborate. -->

設定部分は、もう少し複雑です。

<!-- First, we'll have to configure the timer to operate in one pulse mode. -->

まず最初に、タイマをワンパルスモードで動作するように設定しなければなりません。

``` rust
    // OPM：ワンパルスモードを選択します。
    // CEN：今はカウンタを無効にしておきます。
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
```

<!-- 
Then, we'll like to have the `CNT` counter operate at a frequency of 1 KHz because our `delay`
function takes a number of milliseconds as arguments and 1 KHz produces a 1 millisecond period. For
that we'll have to configure the prescaler.
 -->

次に、`CNT`カウンタが1KHzの周波数で動作するようにします。なぜなら、`delay`関数がミリ秒を引数として取り、1KHzは1ミリ秒の周期を生成するからです。
このために、プリスケーラを設定する必要があります。

``` rust
    // カウンタが1KHzで動作するようにプリスケーラを設定します。
    tim6.psc.write(|w| w.psc().bits(psc));
```

<!-- 
I'm going to let you figure out the value of the prescaler, `psc`. Remember that the frequency of
the counter is `apb1 / (psc + 1)` and that `apb1` is 8 MHz.
 -->

プリスケーラに設定する`psc`の値は、あなたが見つけ出して下さい。
カウンタの周波数は、`apb1 / (psc + 1)`で`apb1`は8MHzであることを思い出して下さい。
