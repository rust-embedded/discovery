# LEDs, again

In the last section, I gave you "initialized" peripherals (I initialized them
before `main`). That's why just writing to `BSRR` was enough to control the
LEDs. But, peripherals are not "initialized" right after the microcontroller
boots.

In this section, you'll have more "fun" with registers: You'll have to configure
`GPIOE` pins as digital outputs so that you'll be able to drive LEDs again.

This is the starter code.

``` rust
#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let (gpioe, rcc) =
        unsafe { (peripheral::gpioe_mut(), peripheral::rcc_mut()) };

    // TODO initialize GPIOE

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8(true)
            .odr9(true)
            .odr10(true)
            .odr11(true)
            .odr12(true)
            .odr13(true)
            .odr14(true)
            .odr15(true)
    });

    loop {}
}
```

If you run the starter code, you'll see that nothing happens this time.
Furthermore, if you print the `GPIOE` register block, you'll see that every
register is "zeroed" even after the `gpioe.odr.write` statement was executed!

```
(gdb) p/x *gpioe
$1 = f3::peripheral::gpio::Gpio {
  moder: f3::peripheral::gpio::Moder {
    register: volatile_register::RW<u32> {
      register: 0x0
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
      register: 0x0
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
