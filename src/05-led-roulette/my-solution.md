# My solution

What solution did you come up with?

Here's mine:

``` rust
{{#include examples/my-solution.rs}}
```

One more thing! Check that your solution also works when compiled in "release" mode:

``` console
$ cargo build --target thumbv7em-none-eabihf --release
```

You can test it with this `gdb` command:

``` console
$ # or, you could simply call `cargo run --target thumbv7em-none-eabihf --release`
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/led-roulette
$ #                                              ~~~~~~~
```

Binary size is something we should always keep an eye on! How big is your solution? You can check
that using the `size` command on the release binary:

``` console
$ # equivalent to size target/thumbv7em-none-eabihf/debug/led-roulette
$ cargo size --target thumbv7em-none-eabihf --bin led-roulette -- -A
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
led-roulette  :
section               size        addr
.vector_table          404   0x8000000
.text                21144   0x8000194
.rodata               3144   0x800542c
.data                    0  0x20000000
.bss                     4  0x20000000
.uninit                  0  0x20000004
.debug_abbrev        19160         0x0
.debug_info         471239         0x0
.debug_aranges       18376         0x0
.debug_ranges       102536         0x0
.debug_str          508618         0x0
.debug_pubnames      76975         0x0
.debug_pubtypes     112797         0x0
.ARM.attributes         58         0x0
.debug_frame         55848         0x0
.debug_line         282067         0x0
.debug_loc             845         0x0
.comment               147         0x0
Total              1673362


$ cargo size --target thumbv7em-none-eabihf --bin led-roulette --release -- -A
    Finished release [optimized + debuginfo] target(s) in 0.03s
led-roulette  :
section              size        addr
.vector_table         404   0x8000000
.text                5380   0x8000194
.rodata               564   0x8001698
.data                   0  0x20000000
.bss                    4  0x20000000
.uninit                 0  0x20000004
.debug_loc           9994         0x0
.debug_abbrev        1821         0x0
.debug_info         74974         0x0
.debug_aranges        600         0x0
.debug_ranges        6848         0x0
.debug_str          52828         0x0
.debug_pubnames     20821         0x0
.debug_pubtypes     18891         0x0
.ARM.attributes        58         0x0
.debug_frame         1088         0x0
.debug_line         15307         0x0
.comment               19         0x0
Total              209601
```

> **NOTE** The Cargo project is already configured to build the release binary using LTO.

Know how to read this output? The `text` section contains the program instructions. It's around 5.25KB
in my case. On the other hand, the `data` and `bss` sections contain variables statically allocated
in RAM (`static` variables). A `static` variable is being used in `aux5::init`; that's why it shows 4
bytes of `bss`.

One final thing! We have been running our programs from within GDB but our programs doesn't depend on
GDB at all. You can confirm this be closing both GDB and OpenOCD and then resetting the board by
pressing the black button on the board. The LED roulette application will run without intervention
of GDB.
