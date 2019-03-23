# `uprintln!`

<!-- 
For the next exercise, we'll implement the `uprint!` family of macros. Your goal is to make this
line of code work:
 -->

次の演習は、`uprint!`系マクロの実装です。目標は、次のコードが動くことです。

``` rust
    uprintln!(serial, "The answer is {}", 40 + 2);
```

<!-- Which must send the string `"The answer is 42"` through the serial interface. -->

これは、シリアルインタフェースを通じて、文字列の`"The answer is 42"`を送信しなければなりません。

<!-- 
How do we go about that? It's informative to look into the `std` implementation of `println!`.
 -->

どうしたら良いのでしょうか？`std`の`println!`実装を見ると、有益な情報が得られます。

``` rust
// src/libstd/macros.rs
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}
```

<!-- 
Looks simple so far. We need the built-in `format_args!` macro (it's implemented in the compiler so we
can't see what it actually does). We'll have to use that macro in the exact same way. What does this
`_print` function do?
 -->

非常に単純に見えます。組込み機能の`format_args!`マクロ（コンパイラに実装されているため、実際に何をしているかは見えません）が必要です。
このマクロを、正しい方法で使わなければなりません。`_print`関数は、何をやっているのでしょうか？

``` rust
// src/libstd/io/stdio.rs
pub fn _print(args: fmt::Arguments) {
    let result = match LOCAL_STDOUT.state() {
        LocalKeyState::Uninitialized |
        LocalKeyState::Destroyed => stdout().write_fmt(args),
        LocalKeyState::Valid => {
            LOCAL_STDOUT.with(|s| {
                if s.borrow_state() == BorrowState::Unused {
                    if let Some(w) = s.borrow_mut().as_mut() {
                        return w.write_fmt(args);
                    }
                }
                stdout().write_fmt(args)
            })
        }
    };
    if let Err(e) = result {
        panic!("failed printing to stdout: {}", e);
    }
}
```

<!-- 
That *looks* complicated but the only part we are interested in is: `w.write_fmt(args)` and
`stdout().write_fmt(args)`. What `print!` ultimately does is call the `fmt::Write::write_fmt` method
with the output of `format_args!` as its argument.
 -->

複雑に*見えます*が、興味のある部分は、`w.write_fmt(args)`と`stdout().write_fmt(args)`だけです。
結局のところ、`print!`が行っていることは、`format_args!`の出力を引数にして、`fmt::Write::write_fmt`メソッドを呼ぶことなのです。

<!-- 
Luckily we don't have to implement the `fmt::Write::write_fmt` method either because it's a default
method. We only have to implement the `fmt::Write::write_str` method.
 -->

幸運なことに、`fmt::Write::write_fmt`メソッドはデフォルトメソッドなので、実装する必要はありません。
`fmt::Write::write_str`メソッドだけを実装しなければなりません。

<!-- Let's do that. -->

では、やってみましょう。

<!-- 
This is what the macro side of the equation looks like. What's left to be done by you is provide the
implementation of the `write_str` method.
 -->

ここまでが、方程式のマクロ側がどのように見えるか、です。残りの取り組まなければならないことは、`write_str`メソッドの実装を提供することです。

<!-- 
Above we saw that `Write` is in `std::fmt`. We don't have access to `std` but `Write` is also
available in `core::fmt`.
 -->

上の方で、`std::fmt`の`Write`を見ました。`std`にはアクセスできませんが、`Write`は`core::fmt`でも利用可能です。

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // TODO ここの実装して下さい
        // ヒント：以前のプログラムと非常に似たものになります
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    let mut serial = SerialPort { usart1 };

    uprintln!(serial, "The answer is {}", 40 + 2);

    loop {}
}
```
