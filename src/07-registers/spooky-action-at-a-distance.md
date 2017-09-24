# Spooky action at a distance

`BSRR` is not the only register that can control the pins of Port E. The `ODR`
register also lets you change the value of the pins. Furthermore, `ODR` also
lets you retrieve the current output status of Port E.

`ODR` is documented in:

> Section 11.4.6 GPIO port output data register - Page 239

Let's try this program:

``` rust
#![no_std]
#![no_main]

#[macro_use]
extern crate pg;
use core::ptr;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    unsafe {
        const GPIOE_BSRR: u32 = 0x4800_1018;
        const GPIOE_ODR: u32 = 0x4800_1014;

        iprintln!("ODR = 0x{:04x}",
                  ptr::read_volatile(GPIOE_ODR as *const u16));

        // Turn on the NORTH LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

        iprintln!("ODR = 0x{:04x}",
                  ptr::read_volatile(GPIOE_ODR as *const u16));

        // Turn on the EAST LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

        iprintln!("ODR = 0x{:04x}",
                  ptr::read_volatile(GPIOE_ODR as *const u16));

        // Turn off the NORTH LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

        iprintln!("ODR = 0x{:04x}",
                  ptr::read_volatile(GPIOE_ODR as *const u16));

        // Turn off the EAST LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
    }

    loop {}
}
```

If you run this program, you'll see:

```
# itmdump's console
ODR = 0x0000
ODR = 0x0200
ODR = 0x0a00
ODR = 0x0800
```

Side effects! Although we are reading the same address multiple times without
actually modifying it, we still see its value change every time `BSRR` is
written to.
