# Type safe manipulation

The last register we were working with, `ODR`, had this in its documentation:

> Bits 31:16 Reserved, must be kept at reset value

We are not supposed to write to those bits of the register or Bad Stuff May Happen.

There's also the fact the registers have different read/write permissions. Some of them are write
only, others can be read and written to and there must be others that are read only.

Finally, directly working with hexadecimal addresses is error prone. You already saw that trying to
access an invalid memory address causes an exception which disrupts the execution of our program.

Wouldn't it be nice if we had an API to manipulate registers in a "safe" manner? Ideally, the API
should encode these three points I've mentioned: No messing around with the actual addresses, should
respect read/write permissions and should prevent modification of the reserved parts of a register.

Well, we do! `aux7::init()` actually returns a value that provides a type safe API to manipulate the
registers of the  `GPIOE` peripheral.

As you may remember: a group of registers associated to a peripheral is called register block, and
it's located in a contiguous region of memory. In this type safe API each register block is modeled
as a `struct` where each of its fields represents a register. Each register field is a different
newtype over e.g. `u32` that exposes a combination of the following methods: `read`, `write` or
`modify` according to its read/write permissions. Finally, these methods don't take primitive values
like `u32`, instead they take yet another newtype that can be constructed using the builder pattern
and that prevent the modification of the reserved parts of the register.

The best way to get familiar with this API is to port our running example to it.

``` rust
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    // Turn on the North LED
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // Turn on the East LED
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // Turn off the North LED
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // Turn off the East LED
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
```

First thing you notice: There are no magic addresses involved. Instead we use a more human friendly
way, for example `gpioe.bsrr`, to refer to the `BSRR` register in the `GPIOE` register block.

Then we have this `write` method that takes a closure. If the identity closure (`|w| w`) is used,
this method will set the register to its *default* (reset) value, the value it had right after the
microcontroller was powered on / reset. That value is `0x0` for the `BSRR` register. Since we want
to write a non-zero value to the register, we use builder methods like `bs9` and `br9` to set some
of the bits of the default value.

Let's run this program! There's some interesting stuff we can do *while* debugging the program.

`gpioe` is a reference to the `GPIOE` register block. `print gpioe` will return the base address of
the register block.

```
$ cargo run
Breakpoint 3, main () at src/07-registers/src/main.rs:9
9           let gpioe = aux7::init().1;

(gdb) next
12          gpioe.bsrr.write(|w| w.bs9().set_bit());

(gdb) print gpioe
$1 = (stm32f30x::gpioc::RegisterBlock *) 0x48001000
```

But if we instead `print *gpioe`, we'll get a *full view* of the register block: the value of each
of its registers will be printed.

```
(gdb) print *gpioe
$2 = stm32f30x::gpioc::RegisterBlock {
  moder: stm32f30x::gpioc::MODER {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0x55550000
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
        value: 0xcc
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

All these newtypes and closures sound like they'd generate large, bloated programs but, if you
actually compile the program in release mode with [LTO] enabled, you'll see that it produces exactly
the same instructions that the "unsafe" version that used `write_volatile` and hexadecimal addresses
did!

[LTO]: https://en.wikipedia.org/wiki/Interprocedural_optimization

``` console
$ cargo objdump --bin registers --release -- -d -no-show-raw-insn -print-imm-hex
registers:      file format ELF32-arm-little

Disassembly of section .text:
main:
 8000188:       bl      #0x22
 800018c:       movw    r0, #0x1018
 8000190:       mov.w   r1, #0x200
 8000194:       movt    r0, #0x4800
 8000198:       str     r1, [r0]
 800019a:       mov.w   r1, #0x800
 800019e:       str     r1, [r0]
 80001a0:       mov.w   r1, #0x2000000
 80001a4:       str     r1, [r0]
 80001a6:       mov.w   r1, #0x8000000
 80001aa:       str     r1, [r0]
 80001ac:       b       #-0x4 <main+0x24>
```

The best part of all this is that I didn't have to write a single line of code to implement the
GPIOE API. All was automatically generated from a System View Description (SVD) file using the
[svd2rust] tool. This SVD file is actually an XML file that microcontroller vendors provide and that
contains the register maps of their microcontrollers. The file contains the layout of register
blocks, the base addresses, the read/write permissions of each register, the layout of the
registers, whether a register has reserved bits and lots of other useful information.

[svd2rust]: https://crates.io/crates/svd2rust
