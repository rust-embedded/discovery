# My solution

What solution did you come up with?

Here's mine:

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate pg;

use core::iter;

use pg::delay;
use pg::led::LEDS;

#[no_mangle]
pub fn main() -> ! {
    loop {
        for (current, next) in LEDS.iter()
            .zip(LEDS.iter().skip(1).chain(iter::once(&LEDS[0]))) {
            next.on();
            delay::ms(50);
            current.off();
            delay::ms(50);
        }
    }
}
```

One more thing! Check that your solution also works when compiled in "release"
mode:

```
$ xargo build --target thumbv7em-none-eabihf --release
```

You can test it with this `gdb` command:

```
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/led-roulette
                                                 ~~~~~~~
```

Binary size is something we should always keep an eye on! How big is your
solution? You can check that using the `size` command on the "release" binary:

```
$ arm-none-eabi-size target/thumbv7em-none-eabihf/*/led-roulette
   text    data     bss     dec     hex filename
   11484    108       0   11592    2d48 target/thumbv7em-none-eabihf/debug/led-roulette
   560        0       0     560     230 target/thumbv7em-none-eabihf/release/led-roulette
```

> **NOTE** The Cargo project is already configured to build the release binary
> using LTO.

Know how to read this output? The `text` section contains the program
instructions. It's around five hundred bytes in my case. On the other hand, the
`data` and `bss` sections contain variables statically allocated in RAM
(`static` variables). I'm not using any so the sizes of these sections are
zero.

One final thing! We have been running our programs from within GDB but our
programs don't depend on GDB at all. You can confirm this be closing both GDB
and OpenOCD and then resetting the board by pressing the black button on the
board. The LED roulette application will run without intervention of GDB.
