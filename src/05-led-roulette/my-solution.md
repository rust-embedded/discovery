# My solution

What solution did you come up with?

Here's mine:

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use nrf51_hal as hal;
use hal::prelude::*;

// All border LEDs in order with the exception of the very first LED which is set
// at the last spot
const COMBINATIONS: [(usize, usize); 16] = [
    (2, 4), (1, 2), (2, 5), (1, 3), (3, 8), (2, 1), (1, 4), (3, 2), (2,6),
    (3, 1), (2, 7), (3, 3), (1, 8), (2, 2), (3, 4), (1, 1)
];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();

    let mut delay = hal::Timer::new(p.TIMER0);

    let p0 = hal::gpio::p0::Parts::new(p.GPIO);

    // Initialize all rows and cols to off
    let mut row1 = p0.p0_13.into_push_pull_output(hal::gpio::Level::Low).degrade();
    let row2 = p0.p0_14.into_push_pull_output(hal::gpio::Level::Low).degrade();
    let row3 = p0.p0_15.into_push_pull_output(hal::gpio::Level::Low).degrade();
    let mut col1 = p0.p0_04.into_push_pull_output(hal::gpio::Level::High).degrade();
    let col2 = p0.p0_05.into_push_pull_output(hal::gpio::Level::High).degrade();
    let col3 = p0.p0_06.into_push_pull_output(hal::gpio::Level::High).degrade();
    let col4 = p0.p0_07.into_push_pull_output(hal::gpio::Level::High).degrade();
    let col5 = p0.p0_08.into_push_pull_output(hal::gpio::Level::High).degrade();
    let col6 = p0.p0_09.into_push_pull_output(hal::gpio::Level::High).degrade();
    let col7 = p0.p0_10.into_push_pull_output(hal::gpio::Level::High).degrade();
    let col8 = p0.p0_11.into_push_pull_output(hal::gpio::Level::High).degrade();
    let col9 = p0.p0_12.into_push_pull_output(hal::gpio::Level::High).degrade();

    // bring up the very first LED
    row1.set_high().unwrap();
    col1.set_low().unwrap();

    let mut cols = [col1, col2, col3, col4, col5, col6, col7, col8, col9];
    let mut rows = [row1, row2, row3];

    loop {
        let mut previous_pair = (1, 1);
        for current_pair in COMBINATIONS.iter() {
            delay.delay_ms(30u32);

            rows[current_pair.0 - 1].set_high().unwrap();
            cols[current_pair.1 - 1].set_low().unwrap();

            rows[previous_pair.0 - 1].set_low().unwrap();
            cols[previous_pair.1 - 1].set_high().unwrap();

            previous_pair = *current_pair;
        }
    }
}
```

One more thing! Check that your solution also works when compiled in "release" mode:

``` console
$ cargo-embed --release
```

If you want to debug your "release" mode binary you'll have to use a different GDB command:

``` console
$ gdb target/thumbv6m-none-eabi/release/led-roulette
```

Binary size is something we should always keep an eye on! How big is your solution? You can check
that using the `size` command on the release binary:

``` console
$ cargo size --bin led-roulette -- -A
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
led-roulette  :
section               size        addr
.vector_table          168         0x0
.text                20996        0xa8
.rodata               2956      0x52ac
.data                    0  0x20000000
.bss                  1088  0x20000000
.uninit                  0  0x20000440
.debug_abbrev        21988         0x0
.debug_info         283389         0x0
.debug_aranges       15832         0x0
.debug_str          307609         0x0
.debug_pubnames      68859         0x0
.debug_pubtypes      55406         0x0
.ARM.attributes         50         0x0
.debug_frame         47732         0x0
.debug_line         199401         0x0
.debug_ranges        68936         0x0
.debug_loc             976         0x0
.comment               147         0x0
Total              1095533


$ cargo size --bin led-roulette --release -- -A
    Finished release [optimized + debuginfo] target(s) in 0.02s
led-roulette  :
section              size        addr
.vector_table         168         0x0
.text                4044        0xa8
.rodata               692      0x1074
.data                   0  0x20000000
.bss                 1076  0x20000000
.uninit                 0  0x20000434
.debug_loc           7520         0x0
.debug_abbrev        3444         0x0
.debug_info         55229         0x0
.debug_aranges       1144         0x0
.debug_ranges        3608         0x0
.debug_str          48267         0x0
.debug_pubnames     15435         0x0
.debug_pubtypes     15970         0x0
.ARM.attributes        50         0x0
.debug_frame         2152         0x0
.debug_line         17050         0x0
.comment              147         0x0
Total              175996
```

> **NOTE** The Cargo project is already configured to build the release binary using LTO.

Know how to read this output? The `text` section contains the program instructions. It's around 4KB
in my case. On the other hand, the `data` and `bss` sections contain variables statically allocated
in RAM (`static` variables).
