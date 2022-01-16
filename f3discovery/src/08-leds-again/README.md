<!-- # LEDs, again -->

# LED、再び

<!-- 
In the last section, I gave you *initialized* (configured) peripherals (I initialized them in
`aux7::init`). That's why just writing to `BSRR` was enough to control the LEDs. But, peripherals
are not *initialized* right after the microcontroller boots.
 -->

前のセクションでは、*初期化済み*（設定済み）のペリフェラルを提供しました（`aux7::init`で初期化していました）。
LEDを制御するために、`BSRR`に書き込むだけで十分だったのは、このおかげです。
しかし、マイクロコントローラが起動した直後、ペリフェラルは*初期化*されていません。

<!-- 
In this section, you'll have more fun with registers. I won't do any initialization and you'll have
to initialize and configure `GPIOE` pins as digital outputs pins so that you'll be able to drive LEDs
again.
 -->

このセクションでは、レジスタを使ってもっとおもしろいことをやります。
私は初期化を行いません。再びLEDを駆動できるようにするために、あなたが`GPIOE`ピンをデジタル出力ピンとして初期化、設定します。

<!-- This is the starter code. -->

スターターコードは下記の通りです。

``` rust
{{#include src/main.rs}}
```

<!-- 
If you run the starter code, you'll see that nothing happens this time. Furthermore, if you print
the `GPIOE` register block, you'll see that every register reads as zero even after the
`gpioe.odr.write` statement was executed!
 -->

スターターコードを動かすと、今回は何も起こりません。その上、`GPIOE`レジスタブロックを表示すると、
`gpioe.odr.write`ステートメントを実行した後でも、全てのレジスタがゼロになっていることがわかるでしょう。

```
$ cargo run
Breakpoint 1, main () at src/08-leds-again/src/main.rs:9
9           let (gpioe, rcc) = aux8::init();

(gdb) continue
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
0x08000f3c in __bkpt ()

(gdb) finish
Run till exit from #0  0x08000f3c in __bkpt ()
main () at src/08-leds-again/src/main.rs:25
25          aux8::bkpt();

(gdb) p/x *gpioe
$1 = stm32f30x::gpioc::RegisterBlock {
  moder: stm32f30x::gpioc::MODER {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  otyper: stm32f30x::gpioc::OTYPER {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  ospeedr: stm32f30x::gpioc::OSPEEDR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  pupdr: stm32f30x::gpioc::PUPDR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  idr: stm32f30x::gpioc::IDR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  odr: stm32f30x::gpioc::ODR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  bsrr: stm32f30x::gpioc::BSRR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  lckr: stm32f30x::gpioc::LCKR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  afrl: stm32f30x::gpioc::AFRL {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  afrh: stm32f30x::gpioc::AFRH {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  brr: stm32f30x::gpioc::BRR {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  }
}
```
