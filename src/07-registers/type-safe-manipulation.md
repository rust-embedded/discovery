# Type safe manipulation

The last register we were working with, `ODR`, had this in its documentation:

> Bits 16:31 Reserved, must be kept at reset value

We are not supposed to write to those bits of the register or Bad Stuff May
Happen.

There's also the fact the registers have different read/write permissions. Some
of them are write only, others can be read and wrote to and there must be others
that are read only.

Finally, directly working with hexadecimal addresses is error prone. You already
saw that trying to access an invalid memory address causes an exception which
disrupts the execution of our program.

Wouldn't it be nice if we had an API to manipulate registers in a "safe" manner?
Ideally, the API should encode these three points I've mentioned: No messing
around with the actual addresses, should respect read/write permissions and
should prevent modification of the reserved parts of a register.

Well, we do! The `pg` crate contains a `peripheral` module that provides such
API.

Each register block is modeled as a `struct` where each field is a register.
Each register is a different newtype over e.g. `u32` and exposes a combination
of the following methods: `read`, `write` or `modify` according to its
read/write permissions. Finally, these methods don't take primitive values like
`u32`, instead they take yet another newtype that can be constructed using the
builder pattern and that prevent the modification of the reserved parts of a
register.

The best way to get familiar with this API is to port our running example to it.

``` rust
#![no_std]
#![no_main]

extern crate pg;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    use pg::peripheral;

    // Get mutable access to the GPIOE register block
    // `unsafe` because this functions hands over (aliases) `&mut-` references
    let gpioe = unsafe { peripheral::gpioe_mut() };

    // Turn on the North LED
    gpioe.bsrr.write(|w| w.bs9(true));

    // Turn on the East LED
    gpioe.bsrr.write(|w| w.bs11(true));

    // Turn off the North LED
    gpioe.bsrr.write(|w| w.br9(true));

    // Turn off the East LED
    gpioe.bsrr.write(|w| w.br11(true));

    loop {}
}
```

First thing you notice: There are no magic addresses involved. Instead we use a
more human friendly: `gpioe.bsrr` to refer to the `BSRR` register in the `GPIOE`
register block.

Then we have this `write` method that takes a closure. If the "identity" closure
is used (`|w| w`), this method will set the register to its "reset value", the
value it had right after the microcontroller was powered on / reset. That value
is `0x0` for the `BSRR` register. Since we want to write a non-zero value to the
register, we use builder methods like `bs9` to set (`true`) or `br9` reset
(`false`) some of the bits of the register value.

Let's run this program! There's some interesting stuff we can do *while*
debugging the program.

`gpioe` is a reference to the `GPIOE` register block. `print gpioe` will return
the base address of the register block.

```
$ (gdb) print gpioe
$1 = (f3::peripheral::gpio::Gpio *) 0x48001000
```

But if we instead `print *gpioe`, we'll get a "full view" of the register block.
The value of each of its registers will be printed. I recommend enabling pretty
print (`set print pretty on`) first, though, to make the output more readable.

```
(gdb) set print pretty on

(gdb) print *gpioe
$2 = f3::peripheral::gpio::Gpio {
  moder: f3::peripheral::gpio::Moder {
    register: volatile_register::RW<u32> {
      register: 0x55550000
    }
  },
  otyper: f3::peripheral::gpio::Otyper {
    register: volatile_register::RW<u32> {
      register: 0x0
    }
  },
  ospeedr: f3::peripheral::gpio::Ospeedr {
    register: volatile_register::RW<u32> {
      register: 0x0
    }
  },
  pupdr: f3::peripheral::gpio::Pupdr {
    register: volatile_register::RW<u32> {
      register: 0x0
    }
  },
  idr: f3::peripheral::gpio::Idr {
    register: volatile_register::RO<u32> {
      register: 0xcc
    }
  },
  odr: f3::peripheral::gpio::Odr {
    register: volatile_register::RW<u32> {
      register: 0x0
    }
  },
  bsrr: f3::peripheral::gpio::Bsrr {
    register: volatile_register::WO<u32> {
      register: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  },
  lckr: f3::peripheral::gpio::Lckr {
    register: volatile_register::RW<u32> {
      register: 0x0
    }
  },
  afrl: f3::peripheral::gpio::Afrl {
    register: volatile_register::RW<u32> {
      register: 0x0
    }
  },
  afrh: f3::peripheral::gpio::Afrh {
    register: volatile_register::RW<u32> {
      register: 0x0
    }
  },
  brr: f3::peripheral::gpio::Brr {
    register: volatile_register::WO<u32> {
      register: core::cell::UnsafeCell<u32> {
        value: 0x0
      }
    }
  }
}
```

All these newtypes and closures sound like they'd generate large, bloated
programs but, if you actually compile the program in release mode with
[LTO](https://en.wikipedia.org/wiki/Interprocedural_optimization)
enabled, you'll see that it produces exactly the same instructions that the
"unsafe" version that used `write_volatile` and hexadecimal addresses did!

```
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/release/registers

080001da <main>:
 80001da:       f241 0018       movw    r0, #4120       ; 0x1018
 80001de:       f44f 7100       mov.w   r1, #512        ; 0x200
 80001e2:       f6c4 0000       movt    r0, #18432      ; 0x4800
 80001e6:       6001            str     r1, [r0, #0]
 80001e8:       f44f 6100       mov.w   r1, #2048       ; 0x800
 80001ec:       6001            str     r1, [r0, #0]
 80001ee:       f04f 7100       mov.w   r1, #33554432   ; 0x2000000
 80001f2:       6001            str     r1, [r0, #0]
 80001f4:       f04f 6100       mov.w   r1, #134217728  ; 0x8000000
 80001f8:       6001            str     r1, [r0, #0]
 80001fa:       e7fe            b.n     80001fa <main+0x20>
```

The best part of all this is that I didn't have to write a single line of code
in the `peripheral` module. All was automatically generated from a System View
Description (SVD) file using the [svd2rust] tool. This SVD file is actually an
XML file that microcontroller vendors provide and that contains the register
maps of their microcontrollers. The file contains the layout of register blocks,
its base addresses, the read/write permissions of each register, the layout of
the registers, whether a register has reserved bits and much more information.

[svd2rust]: https://crates.io/crates/svd2rust
