# My solution

What solution did you come up with?

Here's mine:

``` rust
#![deny(unsafe_code)]
#![no_std]

extern crate aux5;

use aux5::prelude::*;
use aux5::{Delay, Leds};

fn main() {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let ms = 50_u8;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay.delay_ms(ms);
            leds[curr].off();
            delay.delay_ms(ms);
        }
    }
}
```

One more thing! Check that your solution also works when compiled in "release" mode:

``` console
$ xargo build --target thumbv7em-none-eabihf --release
```

You can test it with this `gdb` command:

``` console
$ # or, you could simply call `xargo run --target thumbv7em-none-eabihf --release`
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/led-roulette
$ #                                              ~~~~~~~
```

Binary size is something we should always keep an eye on! How big is your solution? You can check
that using the `size` command on the release binary:

``` console
$ arm-none-eabi-size target/thumbv7em-none-eabihf/*/led-roulette
   text    data     bss     dec     hex filename
  20474       0       4   20478    4ffe target/thumbv7em-none-eabihf/debug/led-roulette
   2148       0       4    2152     868 target/thumbv7em-none-eabihf/release/led-roulette
```

> **NOTE** The Cargo project is already configured to build the release binary using LTO.

Know how to read this output? The `text` section contains the program instructions. It's around 2KB
in my case. On the other hand, the `data` and `bss` sections contain variables statically allocated
in RAM (`static` variables). A `static` variable is being used in `aux5::init`; that's why it shows 4
bytes of `bss`.

One final thing! We have been running our programs from within GDB but our programs don't depend on
GDB at all. You can confirm this be closing both GDB and OpenOCD and then resetting the board by
pressing the black button on the board. The LED roulette application will run without intervention
of GDB.
