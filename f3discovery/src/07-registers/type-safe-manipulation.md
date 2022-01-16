<!-- # Type safe manipulation -->

# 型安全な操作

<!-- The last register we were working with, `ODR`, had this in its documentation: -->

前回の最後に取り扱った`ODR`レジスタは、ドキュメント内で次のように書かれています。

<!-- > Bits 16:31 Reserved, must be kept at reset value -->

> ビット16:31 予約済み, リセット値を保持しなければなりません

<!-- We are not supposed to write to those bits of the register or Bad Stuff May Happen. -->

レジスタのこれらのビットには書き込んではいけないようです。そうでなければ、悪いことが起こるでしょう。

<!-- 
There's also the fact the registers have different read/write permissions. Some of them are write
only, others can be read and wrote to and there must be others that are read only.
 -->

レジスタは、異なる読み書きのパーミッションを持っている、という事実もあります。
書き込み専用のものもあれば、読み書き可能なものもあり、読み込み専用のものもあるはずです。

<!-- 
Finally, directly working with hexadecimal addresses is error prone. You already saw that trying to
access an invalid memory address causes an exception which disrupts the execution of our program.
 -->

最後に、16進数のアドレスを直接扱うことは、間違いを犯しやすいです。
既に不正なメモリアドレスへのアクセスが、プログラムの実行を中断する例外の原因になることを実験しました。

<!-- 
Wouldn't it be nice if we had an API to manipulate registers in a "safe" manner? Ideally, the API
should encode these three points I've mentioned: No messing around with the actual addresses, should
respect read/write permissions and should prevent modification of the reserved parts of a register.
 -->

「安全」な方法でレジスタを操作できるAPIがあると、良いと思いませんか？理想的には、そのAPIは、これまでに述べた3つの点をエンコードするべきです。
実際のアドレスを取り扱わない、読み/書きのパーミッションを守る、レジスタの予約済み部分を修正できないようにする。

<!-- 
Well, we do! `aux7::init()` actually returns a value that provides a type safe API to manipulate the
registers of the  `GPIOE` peripheral.
 -->

やりましょう！実は`aux7::init()`は、`GPIOE`ペリフェラルのレジスタを操作する、型安全なAPIを提供する値を返しています。

<!-- 
As you may remember: a group of registers associated to a peripheral is called register block, and
it's located in a contiguous region of memory. In this type safe API each register block is modeled
as a `struct` where each of its fields represents a register. Each register field is a different
newtype over e.g. `u32` that exposes a combination of the following methods: `read`, `write` or
`modify` according to its read/write permissions. Finally, these methods don't take primitive values
like `u32`, instead they take yet another newtype that can be constructed using the builder pattern
and that prevent the modification of the reserved parts of the register.
 -->

覚えているかもしれませんが、ペリフェラルに関連するレジスタのグループは、レジスタブロックと呼ばれており、連続したメモリ領域に位置しています。
この型安全なAPIでは、各レジスタブロックは、各フィールドがレジスタを表現する`struct`としてモデル化されています。
各レジスタのフィールドは、例えば`u32`の、異なる新しい型で、次のメソッドの組み合わせを提供します。
読み/書きのパーミッションに応じた`read`、`write`、または、`modify`です。
最後に、これらのメソッドは、`u32`のようなプリミティブな値を受け取りません。代わりに、ビルダーパターンを使って構築された、別の新しい型を受け付けます。
このことにより、レジスタの予約済み部分を修正できないようにしています。

<!-- The best way to get familiar with this API is to port our running example to it. -->

このAPIに慣れるための最善の方法は、プログラムを次のように移植することです。

``` rust
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprintln, ITM, RegisterBlock};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    //　北のLEDを点灯
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // 東のLEDを点灯
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // 北のLEDのを消灯
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // 東のLEDを消灯
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
```

<!-- 
First thing you notice: There are no magic addresses involved. Instead we use a more human friendly
way, for example `gpioe.bsrr`, to refer to the `BSRR` register in the `GPIOE` register block.
 -->

最初に気がつくことは、魔法のアドレスがないことです。代わりに、より人間が理解しやすい方法を使っています。
例えば、`gpioe.bsrr`は、`GPIOE`レジスタブロックの`BSRR`レジスタを意味しています。

<!-- 
Then we have this `write` method that takes a closure. If the identity closure (`|w| w`) is used,
this method will set the register to its *default* (reset) value, the value it had right after the
microcontroller was powered on / reset. That value is `0x0` for the `BSRR` register. Since we want
to write a non-zero value to the register, we use builder methods like `bs9` and `br9` to set some
of the bits of the default value.
 -->

そして、`write`メソッドは、クロージャを引数に取ります。アイデンティティクロージャ(`|w| w`)を使った場合、
このメソッドは、レジスタに*デフォルト*（リセット）値を設定します。デフォルト値は、マイクロコントローラが電源オン / リセットされた直後の値です。
`BSRR`レジスタでは、デフォルト値は`0x0`です。
レジスタにゼロでない値を書き込みたいので、デフォルト値のいくつかのビットを設定するために、`bs9`や`br9`のようなビルダーメソッドを使用します。

<!-- Let's run this program! There's some interesting stuff we can do *while* debugging the program. -->

このプログラムを実行してみましょう！プログラムをデバッグしている*間に*いくつかのおもしろいことができます。

<!-- 
`gpioe` is a reference to the `GPIOE` register block. `print gpioe` will return the base address of
the register block.
 -->

`gpioe`は、`GPIOE`レジスタブロックへの参照です。`print gpioe`は、レジスタブロックのベースアドレスを返します。

```
$ cargo run
(..)

Breakpoint 1, registers::__cortex_m_rt_main_trampoline () at src/07-registers/src/main.rs:7
7       #[entry]

(gdb) step
registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:9
9           let gpioe = aux7::init().1;

(gdb) next
12          gpioe.bsrr.write(|w| w.bs9().set_bit());

(gdb) print gpioe
$1 = (*mut stm32f3::stm32f303::gpioc::RegisterBlock) 0x48001000
```

<!-- 
But if we instead `print *gpioe`, we'll get a *full view* of the register block: the value of each
of its registers will be printed.
 -->

しかし、代わりに`print *gpioe`を実行すると、レジスタブロックの*全貌*を得ることができます。
レジスタの各値が表示されます。

```
(gdb) print *gpioe
$2 = stm32f3::stm32f303::gpioc::RegisterBlock {
  moder: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_MODER> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 1431633920
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_MODER>
  },
  otyper: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_OTYPER> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_OTYPER>
  },
  ospeedr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_OSPEEDR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_OSPEEDR>
  },
  pupdr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_PUPDR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_PUPDR>
  },
  idr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_IDR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 204
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_IDR>
  },
  odr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_ODR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_ODR>
  },
  bsrr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_BSRR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_BSRR>
  },
  lckr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_LCKR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_LCKR>
  },
  afrl: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_AFRL> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_AFRL>
  },
  afrh: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_AFRH> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_AFRH>
  },
  brr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_BRR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_BRR>
  }
}
```

<!-- 
All these newtypes and closures sound like they'd generate large, bloated programs but, if you
actually compile the program in release mode with [LTO] enabled, you'll see that it produces exactly
the same instructions that the "unsafe" version that used `write_volatile` and hexadecimal addresses
did!
 -->

全てのこれらの新しい型とクロージャは、大きく肥大化したプログラムを生成するように見えます。
しかし、実際にこのプログラムを[LTO]を有効化してリリースモードでコンパイルすると、
`write_volatile`と16進数アドレスを使った「unsafe」版と全く同じ命令が生成されることがわかります。

[LTO]: https://en.wikipedia.org/wiki/Interprocedural_optimization

Use `cargo objdump` to grab the assembler code to `release.txt`:
``` console
cargo objdump --bin registers --release -- -d --no-show-raw-insn --print-imm-hex > release.txt
```

Then search for `main` in `release.txt`
```
0800023e <main>:
 800023e:      	push	{r7, lr}
 8000240:      	mov	r7, sp
 8000242:      	bl	#0x2
 8000246:      	trap

08000248 <registers::__cortex_m_rt_main::h199f1359501d5c71>:
 8000248:      	push	{r7, lr}
 800024a:      	mov	r7, sp
 800024c:      	bl	#0x22
 8000250:      	movw	r0, #0x1018
 8000254:      	mov.w	r1, #0x200
 8000258:      	movt	r0, #0x4800
 800025c:      	str	r1, [r0]
 800025e:      	mov.w	r1, #0x800
 8000262:      	str	r1, [r0]
 8000264:      	mov.w	r1, #0x2000000
 8000268:      	str	r1, [r0]
 800026a:      	mov.w	r1, #0x8000000
 800026e:      	str	r1, [r0]
 8000270:      	b	#-0x4 <registers::__cortex_m_rt_main::h199f1359501d5c71+0x28>
```

<!-- 
The best part of all this is that I didn't have to write a single line of code to implement the
GPIOE API. All was automatically generated from a System View Description (SVD) file using the
[svd2rust] tool. This SVD file is actually an XML file that microcontroller vendors provide and that
contains the register maps of their microcontrollers. The file contains the layout of register
blocks, the base addresses, the read/write permissions of each register, the layout of the
registers, whether a register has reserved bits and lots of other useful information.
 -->

最も良い点は、GPIOE APIを実装するために、1行もコードを書く必要がなかったことです。
全ては、[svd2rust]ツールを使って、System View Description (SVD)ファイルから自動生成されています。
SVDファイルは、実のところ、マイクロコントローラのベンダが提供しているXMLファイルです。このファイルは、マイクロコントローラのレジスタマップを含んでいます。
このファイルは、レジスタブロックのレイアウトやベースアドレス、各レジスタの読み/書きのパーミッション、レジスタのレイアウト、
レジスタが予約済みのビットを持っているかどうか、などの有用な情報を含んでいます。

[svd2rust]: https://crates.io/crates/svd2rust
