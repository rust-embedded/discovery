# LEDs, again

In the last section, I gave you *initialized* (configured) peripherals (I initialized them in
`aux7::init`). That's why just writing to `BSRR` was enough to control the LEDs. But, peripherals
are not *initialized* right after the microcontroller boots.

In this section, you'll have more fun with registers. I won't do any initialization and you'll have
to initialize configure `GPIOE` pins as digital outputs pins so that you'll be able to drive LEDs
again.

This is the starter code.

``` rust
{{#include src/main.rs}}
```

If you run the starter code, you'll see that nothing happens this time. Furthermore, if you print
the `GPIOE` register block, you'll see that every register reads as zero even after the
`gpioe.odr.write` statement was executed!

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
