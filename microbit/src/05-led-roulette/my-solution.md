# My solution

What solution did you come up with?

Here's mine, it's probably one of the simplest (but of course not most
beautiful) way to generate the required matrix:

``` rust
{{#include examples/my-solution.rs}}
```

One more thing! Check that your solution also works when compiled in "release" mode:

``` console
# For micro:bit v2
$ cargo embed --features v2 --target thumbv7em-none-eabihf --release
  (...)

# For micro:bit v1
$ cargo embed --features v1 --target thumbv6m-none-eabi --release
  (...)
```

If you want to debug your "release" mode binary you'll have to use a different GDB command to load the other binary:

``` console
# For micro:bit v2
$ gdb target/thumbv7em-none-eabihf/release/led-roulette

# For micro:bit v1
$ gdb target/thumbv6m-none-eabi/release/led-roulette
```

Binary size is something we should always keep an eye on! How big is your solution? You can check
that using the `size` command on the release binary:

``` console
# For micro:bit v2
$ cargo size --features v2 --target thumbv7em-none-eabihf -- -A
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
led-roulette  :
section               size        addr
.vector_table          256         0x0
.text                33564       0x100
.rodata               4824      0x6a68
.data                    0  0x20000000
.gnu.sgstubs             0      ox9700
.bss                  1092  0x20000000
.uninit                  0  0x20000444
.debug_loc            5446         0x0
.debug_abbrev        22709         0x0
.debug_info         630006         0x0
.debug_aranges       22488         0x0
.debug_ranges       186616         0x0
.debug_str          726748         0x0
.comment                64         0x0
.ARM.attributes         58         0x0
.debug_frame         71712         0x0
.debug_line         320979         0x0
.debug_pubnames        702         0x0
.debug_pubtypes         71         0x0
Total              2027335


$ cargo size --features v2 --target thumbv7em-none-eabihf --release -- -A
    Finished release [optimized + debuginfo] target(s) in 0.02s
led-roulette  :
section              size        addr
.vector_table         256         0x0
.text                6516       0x100
.rodata               612      0x19bc
.data                   0  0x20000000
.gnu.sgstubs            0      0x1ce0
.bss                 1076  0x20000000
.uninit                 0  0x20000434
.debug_loc          10784         0x0
.debug_abbrev        3159         0x0
.debug_info         63612         0x0
.debug_aranges       1040         0x0
.debug_ranges       11576         0x0
.debug_str          69813         0x0
.comment               64         0x0
.ARM.attributes        58         0x0
.debug_frame         2084         0x0
.debug_line         18180         0x0
.debug_pubnames       702         0x0
.debug_pubtypes        71         0x0
Total              189603

# micro:bit v1
$ cargo size --features v1 --target thumbv6m-none-eabi -- -A
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
led-roulette  :
section               size        addr
.vector_table          168         0x0
.text                28584        0xa8
.rodata               2948      0x7050
.data                    0  0x20000000
.bss                  1092  0x20000000
.uninit                  0  0x20000444
.debug_abbrev        30020         0x0
.debug_info         373392         0x0
.debug_aranges       18344         0x0
.debug_ranges        89656         0x0
.debug_str          375887         0x0
.debug_pubnames     115633         0x0
.debug_pubtypes      86658         0x0
.ARM.attributes         50         0x0
.debug_frame         54144         0x0
.debug_line         237714         0x0
.debug_loc            1499         0x0
.comment               109         0x0
Total              1415898

$ cargo size --features v1 --target thumbv6m-none-eabi --release -- -A
    Finished release [optimized + debuginfo] target(s) in 0.02s
led-roulette  :
section              size        addr
.vector_table         168         0x0
.text                4848        0xa8
.rodata               648      0x1398
.data                   0  0x20000000
.bss                 1076  0x20000000
.uninit                 0  0x20000434
.debug_loc           9705         0x0
.debug_abbrev        3235         0x0
.debug_info         61908         0x0
.debug_aranges       1208         0x0
.debug_ranges        5784         0x0
.debug_str          57358         0x0
.debug_pubnames     22959         0x0
.debug_pubtypes     18891         0x0
.ARM.attributes        50         0x0
.debug_frame         2316         0x0
.debug_line         18444         0x0
.comment               19         0x0
Total              208617

```

> **NOTE** The Cargo project is already configured to build the release binary using LTO.

Know how to read this output? The `text` section contains the program instructions. On the other hand,
the `data` and `bss` sections contain variables statically allocated in RAM (`static` variables).
If you remember back in the specification of the microcontroller on your micro:bit, you should
notice that its flash memory is actually far too small to contain this binary, so how is this possible?
As we can see from the size statistics most of the binary is actually made up of debugging related
sections, those are however not flashed to the microcontroller at any time, after all they aren't
relevant for the execution.
