<!-- # The `Led` and `Delay` abstractions -->

# `Led`と`Delay`の抽象化

<!-- 
Now, I'm going to introduce two high level abstractions that we'll use to implement the LED roulette
application.
 -->

これから、LEDルーレットアプリケーションを実装するための、2つの高レベルな抽象化を紹介します。

<!-- 
The auxiliary crate, `aux5`, exposes an initialization function called `init`. When called this
function returns two values packed in a tuple: a `Delay` value and a `Leds` value.
 -->

補助クレートの`aux5`は、`init`という初期化関数を公開しています。
この関数を呼び出すと、`Delay`と`Leds`の値からなるタプルが返ってきます。

<!-- `Delay` can be used to block your program for a specified amount of milliseconds. -->

`Delay`は、ミリ秒単位で指定された時間の間、プログラムをブロックします。

<!-- 
`Leds` is actually an array of eight `Led`s. Each `Led` represents one of the LEDs on the F3 board,
and exposes two methods: `on` and `off` which can be used to turn the LED on or off, respectively.
 -->

`Leds`は、8個の`Led`からなる配列です。各`Led`は、F3ボード上のLEDの1つを表しています。
そして、`on`と`off`という2つのメソッドを公開しており、それぞれ、LEDをオンまたはオフにします。

<!-- Let's try out these two abstractions by modifying the starter code to look like this: -->

スターターコードを次のように修正して、2つの抽象化を試してみましょう。

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period = 500_u16;

    loop {
        leds[0].on();
        delay.delay_ms(half_period);

        leds[0].off();
        delay.delay_ms(half_period);
    }
}
```

<!-- Now build it: -->

ビルドします。

``` console
$ cargo build --target thumbv7em-none-eabihf
```

<!-- 
> **NOTE** It's possible to forget to rebuild the program *before* starting a GDB session; this
> omission can lead to very confusing debug sessions. To avoid this problem you can call `cargo run`
> instead of `cargo build`; `cargo run` will build *and* start a debug session ensuring you never
> forget to recompile your program.
 -->

> **注記** GDBセッションを開始する*前*に、プログラムを再ビルドすることを忘れることがあります。このうっかりミスは、非常に混乱するデバッグセッションを作り上げます。
> この問題を避けるために、`cargo build`ではなく`cargo run`を呼び出すことができます。
> `cargo run`は、ビルド*と*デバッグセッションの開始を行い、プログラムの再コンパイル忘れが起きないようにしてくれます。

<!-- Now, we'll repeat the flashing procedure that we did in the previous section: -->

次に、前のセクションで行ったとおり、Flashへの書き込み手順を繰り返します。

``` console
$ # これはプログラムのGDBセッションを開始します。バイナリのパスを指定する必要はありません。
$ arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...done.
(gdb) target remote :3333
Remote debugging using :3333
(..)

(gdb) load
Loading section .vector_table, size 0x188 lma 0x8000000
Loading section .text, size 0x3fc6 lma 0x8000188
Loading section .rodata, size 0xa0c lma 0x8004150
Start address 0x8000188, load size 19290
Transfer rate: 19 KB/sec, 4822 bytes/write.

(gdb) break main
Breakpoint 1 at 0x800018c: file src/05-led-roulette/src/main.rs, line 9.

(gdb) continue
Continuing.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, main () at src/05-led-roulette/src/main.rs:9
9           let (mut delay, mut leds): (Delay, Leds) = aux5::init();
```

<!-- 
OK. Let's step through the code. This time, we'll use the `next` command instead of `step`. The
difference is that the `next` command will step *over* function calls instead of going inside them.
 -->

では、コードをステップ実行していきましょう。今回は、`step`の代わりに、`next`コマンドを使います。
`next`は、関数呼び出し時、関数内に入らずに、ステップ*オーバー*します。

```
(gdb) next
11          let half_period = 500_u16;

(gdb) next
13          loop {

(gdb) next
14              leds[0].on();

(gdb) next
15              delay.delay_ms(half_period);
```

<!-- 
After executing the `leds[0].on()` statement, you should see a red LED, the one pointing North,
turn on.
 -->

`leds[0].on()`ステートメント実行後、北を指し示す赤いLEDが点灯するはずです。

<!-- Let's continue stepping over the program: -->

プログラムのステップオーバー実行を続けます。

```
(gdb) next
17              leds[0].off();

(gdb) next
18              delay.delay_ms(half_period);
```

<!-- 
The `delay_ms` call will block the program for half a second but you may not notice because the
`next` command also takes some time to execute. However, after stepping over the `leds[0].off()`
statement you should see the red LED turn off.
 -->

`delay_ms`の呼び出しは、0.5秒の間プログラムをブロックしますが、それに気づかないかもしれません。
`next`コマンドの実行にいくらか時間がかかるためです。
しかし、`leds[0].off()`ステートメントをステップオーバーすると、赤いLEDが消灯するでしょう。

<!-- You can already guess what this program does. Let it run uninterrupted using the `continue` command. -->

すでに、このプログラムが何をするか、予測できているでしょう。`continue`コマンドを使って、中断せずに実行しましょう。

```
(gdb) continue
Continuing.
```

<!-- 
Now, let's do something more interesting. We are going to modify the behavior of our program using
GDB.
 -->

次は、もっと面白いことをやります。GDBを使って、プログラムの動作を変更します。

<!-- 
First, let's stop the infinite loop by hitting `Ctrl+C`. You'll probably end up somewhere inside
`Led::on`, `Led::off` or `delay_ms`:
 -->

まずは、`Ctrl+C`を入力し、無限ループを停止します。`Led::on`か、`Led::off`か`delay_ms`内のどこかに居るでしょう。

```
Program received signal SIGINT, Interrupt.
0x080033f6 in core::ptr::read_volatile (src=0xe000e010) at /checkout/src/libcore/ptr.rs:472
472     /checkout/src/libcore/ptr.rs: No such file or directory.
```

<!-- 
In my case, the program stopped its execution inside a `read_volatile` function. GDB output shows
some interesting information about that: `core::ptr::read_volatile (src=0xe000e010)`. This means
that the function comes from the `core` crate and that it was called with argument `src =
0xe000e010`.
 -->

私の場合、`read_volatile`関数の中で、プログラムの実行が停止していました。
GDBの出力は、`core::ptr::read_volatile (src=0xe000e010)`という面白い情報を示しています。
これは、この関数が`core`クレートから来ており、`src = 0xe000e010`という引数で呼び出されていることを意味します。

<!-- 
Just so you know, a more explicit way to show the arguments of a function is to use the `info args`
command:
 -->

ご存知の通り、関数の引数を表示するための、より明確な方法は、`info args`コマンドを使うことです。

```
(gdb) info args
src = 0xe000e010
```

<!-- 
Regardless of where your program may have stopped you can always look at the output of the
`backtrace` command (`bt` for short) to learn how it got there:
 -->

どこでプログラムが停止したかに関わらず、`backtrace`コマンド（`bt`という省略形があります）を使って、
どのようにそこに到達したか、を見ることができます。

```
(gdb) backtrace
#0  0x080033f6 in core::ptr::read_volatile (src=0xe000e010)
    at /checkout/src/libcore/ptr.rs:472
#1  0x08003248 in <vcell::VolatileCell<T>>::get (self=0xe000e010)
    at $REGISTRY/vcell-0.1.0/src/lib.rs:43
#2  <volatile_register::RW<T>>::read (self=0xe000e010)
    at $REGISTRY/volatile-register-0.2.0/src/lib.rs:75
#3  cortex_m::peripheral::syst::<impl cortex_m::peripheral::SYST>::has_wrapped (self=0x10001fbc)
    at $REGISTRY/cortex-m-0.5.7/src/peripheral/syst.rs:124
#4  0x08002d9c in <stm32f30x_hal::delay::Delay as embedded_hal::blocking::delay::DelayUs<u32>>::delay_us (self=0x10001fbc, us=500000)
    at $REGISTRY/stm32f30x-hal-0.2.0/src/delay.rs:58
#5  0x08002cce in <stm32f30x_hal::delay::Delay as embedded_hal::blocking::delay::DelayMs<u32>>::delay_ms (self=0x10001fbc, ms=500)
    at $REGISTRY/stm32f30x-hal-0.2.0/src/delay.rs:32
#6  0x08002d0e in <stm32f30x_hal::delay::Delay as embedded_hal::blocking::delay::DelayMs<u16>>::delay_ms (self=0x10001fbc, ms=500)
    at $REGISTRY/stm32f30x-hal-0.2.0/src/delay.rs:38
#7  0x080001ee in main () at src/05-led-roulette/src/main.rs:18
```

<!-- `backtrace` will print a trace of function calls from the current function down to main. -->

`backtrace`は、現在の関数からmainまでの関数呼び出しのトレースを表示します。

<!-- 
Back to our topic. To do what we are after, first, we have to return to the `main` function. We can
do that using the `finish` command. This command resumes the program execution and stops it again
right after the program returns from the current function. We'll have to call it several times.
 -->

トピックに戻ります。やりたいことをやるためには、まず、`main`関数に戻る必要が有ります。
`finish`コマンドを使うことで、これができます。このコマンドは、プログラムの実行を再開し、プログラムが現在の関数から戻った直後に停止します。
複数回、このコマンドを呼び出します。

```
(gdb) finish
cortex_m::peripheral::syst::<impl cortex_m::peripheral::SYST>::has_wrapped (self=0x10001fbc)
    at $REGISTRY/cortex-m-0.5.7/src/peripheral/syst.rs:124
124             self.csr.read() & SYST_CSR_COUNTFLAG != 0
Value returned is $1 = 5

(gdb) finish
Run till exit from #0  cortex_m::peripheral::syst::<impl cortex_m::peripheral::SYST>::has_wrapped (
    self=0x10001fbc)
    at $REGISTRY/cortex-m-0.5.7/src/peripheral/syst.rs:124
0x08002d9c in <stm32f30x_hal::delay::Delay as embedded_hal::blocking::delay::DelayUs<u32>>::delay_us (
    self=0x10001fbc, us=500000)
    at $REGISTRY/stm32f30x-hal-0.2.0/src/delay.rs:58
58              while !self.syst.has_wrapped() {}
Value returned is $2 = false

(..)

(gdb) finish
Run till exit from #0  0x08002d0e in <stm32f30x_hal::delay::Delay as embedded_hal::blocking::delay::DelayMs<u16>>::delay_ms (self=0x10001fbc, ms=500)
    at $REGISTRY/stm32f30x-hal-0.2.0/src/delay.rs:38
0x080001ee in main () at src/05-led-roulette/src/main.rs:18
18              delay.delay_ms(half_period);
```

<!-- We are back in `main`. We have a local variable in here: `half_period` -->

`main`に戻ってきました。`half_period`というローカル変数があります。

```
(gdb) info locals
half_period = 500
delay = (..)
leds = (..)
```

<!-- Now, we are going to modify this variable using the `set` command: -->

次に、`set`コマンドを使って、この変数を書き換えます。

```
(gdb) set half_period = 100

(gdb) print half_period
$1 = 100
```

<!-- 
If you let program run free again using the `continue` command, you should see that the LED will
blink at a much faster rate now!
 -->

`continue`コマンドを使って、プログラムの実行すると、LEDが前より速く点滅するはずです！

<!-- 
Question! What happens if you keep lowering the value of `half_period`? At what value of
`half_period` you can no longer see the LED blink?
 -->

質問です！`half_period`の値を下げ続けるとどうなるでしょうか？
`half_period`の値がいくつになると、LEDの点滅が見えなくなるでしょうか？

<!-- Now, it's your turn to write a program. -->

次は、あなたがプログラムを書く番です。
