# LEDs, again

In the last section, I gave you *initialized* (configured) peripherals (I initialized them in
`aux8::init`). That's why just writing to `BSRR` was enough to control the LEDs. But, peripherals
are not *initialized* right after the microcontroller boots.

In this section, you'll have more fun with registers. I won't do any initialization and you'll have
to initialize configure `GPIOE` pins as digital outputs pins so that you'll be able to drive LEDs
again.

This is the starter code.

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate aux8;
#[macro_use]
extern crate cortex_m_rt;

entry!(main);

fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // TODO initialize GPIOE

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    aux8::bkpt();

    loop {}
}
```

If you run the starter code, you'll see that nothing happens this time. Furthermore, if you print
the `GPIOE` register block, you'll see that every register reads as zero even after the
`gpioe.odr.write` statement was executed!

```
(gdb) continue
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
__bkpt () at asm/bkpt.s:3
3         bkpt

(gdb) finish
Run till exit from #0  __bkpt () at asm/bkpt.s:3
leds_again::main () at src/08-leds-again/src/main.rs:28
28          aux8::bkpt();

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
