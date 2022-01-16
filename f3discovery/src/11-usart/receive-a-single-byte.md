<!-- # Receive a single byte -->

# 1バイト受信

<!-- 
So far we have sending data from the micro to your laptop. It's time to try the opposite: receiving
data from your laptop.
 -->

これまで、マイクロコントローラからノートPCへデータを送ってきました。反対側をやってみましょう。
ノートPCからデータを受信します。

<!-- 
There's a `RDR` register that will be filled with the data that comes from the RX line. If we read
that register, we'll retrieve the data that the other side of the channel sent. The question is: How
do we know that we have received (new) data? The status register, `ISR`, has a bit for that purpose:
`RXNE`. We can just busy wait on that flag.
 -->

RX線から来たデータが詰められる`RDP`レジスタがあります。このレジスタを読むと、対向側が送ったデータを取り出すことができます。
（新しい）データを受信したことをどうやって知るか？という疑問があります。`ISR`というステータスレジスタが、この目的のための`RXNE`ビットを持っています。
単純にこのフラグをビジーウェイトすることができます。

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    loop {
        // データが利用可能になるまで待ちます
        while usart1.isr.read().rxne().bit_is_clear() {}

        // データを取り出します
        let _byte = usart1.rdr.read().rdr().bits() as u8;

        aux11::bkpt();
    }
}
```

<!-- 
Let's try this program! Let it run free using `continue` and then type a single character in
minicom/PuTTY's console. What happens? What are the contents of the `_byte` variable?
 -->

このプログラムを試してみて下さい！`continue`を使って実行し、minicom/PuTTYのコンソールから1文字入力します。
何が起こりましたか？`_byte`変数の中身はなんでしょうか？

```
(gdb) continue
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
0x8003d48 in __bkpt ()

(gdb) finish
Run till exit from #0  0x8003d48 in __bkpt ()
usart::main () at src/11-usart/src/main.rs:19
19              aux11::bkpt();

(gdb) p/c _byte
$1 = 97 'a'
```
