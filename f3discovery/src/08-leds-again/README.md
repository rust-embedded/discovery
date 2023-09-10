# 再一次LED

在上一节中，我为您提供了*初始化* (配置) 的外设 (我在`aux7::init`中初始化了它们）。 这就是为什么仅向
`BSRR`写入就足以控制LED的原因。但是，外围设备在微控制器启动后不会立即初始化。

在本节中，您将获得更多寄存器的乐趣。我不会进行任何初始化，您必须将`GPIOE`引脚初始化并配置为数字输出引脚，以便您能够再次驱动LED。

这是启动代码。

``` rust
{{#include src/main.rs}}
```

如果您运行启动程序代码，您将看到这次没有发生任何事情。此外，如果您打印`GPIOE`寄存器块，您将看
到即使在执行`gpioe.odr.write`语句之后，每个寄存器都读取为零！

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
