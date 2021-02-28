# Spooky action at a distance

`BSRR` is not the only register that can control the pins of Port E. The `ODR` register also lets
you change the value of the pins. Furthermore, `ODR` also lets you retrieve the current output
status of Port E.

`ODR` is documented in:

> Section 11.4.6 GPIO port output data register - Page 239

Let's look at this program. The key to this program
is `fn iprint_odr`. This function prints the current
value in `ODR` to the `ITM` console

``` rust
#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprintln, ITM};

// Print the current contents of odr
fn iprint_odr(itm: &mut ITM) {
    const GPIOE_ODR: u32 = 0x4800_1014;

    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );
    }
}

#[entry]
fn main() -> ! {
    let mut itm= aux7::init().0;

    unsafe {
        // A magic addresses!
        const GPIOE_BSRR: u32 = 0x4800_1018;

        // Print the initial contents of ODR
        iprint_odr(&mut itm);

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        iprint_odr(&mut itm);

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        iprint_odr(&mut itm);

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        iprint_odr(&mut itm);

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
        iprint_odr(&mut itm);
    }

    loop {}
}
```

If you run this program
```
$ cargo run
(..)
Breakpoint 1, registers::__cortex_m_rt_main_trampoline () at src/07-registers/src/main.rs:22
22      #[entry]

(gdb) continue
Continuing.
```

You'll see on itmdump's console:

``` console
$ # itmdump's console
(..)
ODR = 0x0000
ODR = 0x0200
ODR = 0x0a00
ODR = 0x0800
ODR = 0x0000
```

Side effects! Although we are reading the same address multiple times without actually modifying it,
we still see its value change every time `BSRR` is written to.
