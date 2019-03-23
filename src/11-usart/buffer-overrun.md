<!-- # Overruns -->

# オーバーラン

<!-- If you wrote your program like this: -->

プログラムを次のように書いた場合、

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // 文字列を送信します
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
    }

    loop {}
}
```

<!-- 
You probably received something like this on your laptop when you executed the program compiled in
debug mode.
 -->

デバッグモードでコンパイルしたプログラムを実行した場合、ノートPCでは、おそらく次のようなものを受信したと思います。

``` console
$ # minicom's terminal
(..)
The uic brwn oxjums oer helaz do.
```

<!-- And if you compiled in release mode, you probably only got something like this: -->

そして、リリースモードでコンパイルした場合、次のような表示になったと思います。

``` console
$ # minicom's terminal
(..)
T
```

<!-- What went wrong? -->

何がいけなかったのでしょう？

<!-- 
You see, sending bytes over the wire takes a relatively large amount of time. I already did the math
so let me quote myself:
 -->

おわかりのように、ワイヤを介してバイトを送信するには、比較的長い時間がかかります。計算しておいた値を引用します。

<!-- 
> With a common configuration of 1 start bit, 8 bits of data, 1 stop bit and a baud rate of 115200
> bps one can, in theory, send 11,520 frames per second. Since each one frame carries a byte of data
> that results in a data rate of 11.52 KB/s
 -->

> 一般的な、スタートビット1ビット、データ8ビット、ストップビット1ビットでボーレートが115200 bpsの設定では、
> 理論上は、毎秒11,520フレームを送信できます。1フレームにつき1バイトのデータを転送するので、データレートは、11.52 KB/秒になります。

<!-- 
Our pangram has a length of 45 bytes. That means it's going to take, at least, 3,900 microseconds
(`45 bytes / (11,520 bytes/s) = 3,906 us`) to send the string. The processor is working at 8 MHz,
where executing an instruction takes 125 nanoseconds, so it's likely going to be done with the `for`
loop is less than 3,900 microseconds.
 -->

送信した文字列は、45バイトの長さです。これは、この文字列を送るために、
少なくとも3,900マイクロ秒（`45 bytes / (11,520 bytes/s) = 3,906 us`）かかることを意味しています。
プロセッサは、8MHzで動作しており、1命令125ナノ秒で実行します。この`for`ループは、3,900マイクロ秒より短い時間で完了するようです。

<!-- 
We can actually time how long it takes to execute the `for` loop. `aux11::init()` returns a
`MonoTimer` (monotonic timer) value that exposes an `Instant` API that's similar to the one in
`std::time`.
 -->

この`for`ループの実行に、どのくらい時間がかかっているのか、を実際に計測します。
`aux11::init()`は、`MonoTimer` (monotonic timer) の値を返します。これは、`std::time`に似た`Instant` APIを提供します。

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    let instant = mono_timer.now();
    // 文字列を送信します
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
    }
    let elapsed = instant.elapsed(); // ティック単位で

    iprintln!(
        &mut itm.stim[0],
        "`for` loop took {} ticks ({} us)",
        elapsed,
        elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );

    loop {}
}
```

<!-- In debug mode, I get: -->

デバッグモードでは、下記の結果が得られました。

``` console
$ # itmdump terminal
(..)
`for` loop took 22415 ticks (2801.875 us)
```

<!-- 
This is less than 3,900 microseconds but it's not that far off and that's why only a few bytes of
information are lost.
 -->

これは、3,900マイクロ秒より短いですが、それほどかけ離れているわけではありません。そのため、情報の数バイトだけが失われました。

<!-- 
In conclusion, the processor is trying to send bytes at a faster rate than what the hardware can
actually handle and this results in data loss. This condition is known as buffer *overrun*.
 -->

結論として、プロセッサは、ハードウェアの実際の処理能力より速いレートでバイトを送信し、その結果、データが失われました。
この状態は、バッファ*オーバーラン*と呼ばれています。

<!-- 
How do we avoid this? The status register (`ISR`) has a flag, `TXE`, that indicates if it's "safe"
to write to the `TDR` register without incurring in data loss.
 -->

どうすれば、これを回避できるでようか？ステータスレジスタ（`ISR`）は、`TXE`というフラグを持っています。
このフラグは、`TDR`レジスタにデータ損失なしで「安全」に書き込むことができるかどうかを、示しています。

<!-- Let's use that to slowdown the processor. -->

プロセッサを減速させるために、これを使いましょう。

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    let instant = mono_timer.now();
    // 文字列を送信します
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        // TDRへの書き込みが安全になるまで待ちます
        while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!

        usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
    }
    let elapsed = instant.elapsed(); // ティック単位で

    iprintln!(
        &mut itm.stim[0],
        "`for` loop took {} ticks ({} us)",
        elapsed,
        elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );

    loop {}
}
```

<!-- 
This time, running the program in debug or release mode should result in a complete string on the
receiving side.
 -->

今回は、デバッグモードかリリースモードかに関わらず、プログラムを実行すると、受信側で完全に文字列を受信できるはずです。

``` console
$ # minicom/PuTTY's console
(..)
The quick brown fox jumps over the lazy dog.
```

<!-- 
The timing of the `for` loop should be closer to the theoretical 3,900 microseconds as well. The
timing below is for the debug version.
 -->

`for`ループの実行時間は、理論上の3,900マイクロ秒に近くなるはずです。
デバッグモードのバージョンを下記に示します。

``` console
$ # itmdump terminal
(..)
`for` loop took 30499 ticks (3812.375 us)
```
