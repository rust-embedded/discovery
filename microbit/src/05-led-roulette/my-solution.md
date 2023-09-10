# 我的解决方案

你想出了什么解决方案？

这是我的，这可能是生成所需矩阵的最简单（但当然不是最漂亮）方法之一：

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

const PIXELS: [(usize, usize); 16] = [
    (0,0), (0,1), (0,2), (0,3), (0,4), (1,4), (2,4), (3,4), (4,4),
    (4,3), (4,2), (4,1), (4,0), (3,0), (2,0), (1,0)
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut last_led = (0,0);

    loop {
        for current_led in PIXELS.iter() {
            leds[last_led.0][last_led.1] = 0;
            leds[current_led.0][current_led.1] = 1;
            display.show(&mut timer, leds, 30);
            last_led = *current_led;
        }
    }
}
```

还有一件事！检查您的解决方案在"release"模式下编译时是否也有效：

``` console
# For micro:bit v2
$ cargo embed --features v2 --target thumbv7em-none-eabihf --release
  (...)

# For micro:bit v1
$ cargo embed --features v1 --target thumbv6m-none-eabi --release
  (...)
```

如果要调试"release"模式二进制文件，则必须使用不同的 GDB 命令：

``` console
# For micro:bit v2
$ gdb target/thumbv7em-none-eabihf/release/led-roulette

# For micro:bit v1
$ gdb target/thumbv6m-none-eabi/release/led-roulette
```

二进制大小是我们应该时刻关注的！你的解决方案有多大？您可以使用`size`发布二进制文件上的命令进行检查：

``` console
# For micro:bit v2
$ cargo size --features v2 --target thumbv7em-none-eabihf -- -A
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
led-roulette  :
section               size        addr
.vector_table          256         0x0
.text                26984       0x100
.rodata               2732      0x6a68
.data                    0  0x20000000
.bss                  1092  0x20000000
.uninit                  0  0x20000444
.debug_abbrev        33941         0x0
.debug_info         494113         0x0
.debug_aranges       23528         0x0
.debug_ranges       130824         0x0
.debug_str          498781         0x0
.debug_pubnames     143351         0x0
.debug_pubtypes     124464         0x0
.ARM.attributes         58         0x0
.debug_frame         69128         0x0
.debug_line         290580         0x0
.debug_loc            1449         0x0
.comment               109         0x0
Total              1841390


$ cargo size --features v2 --target thumbv7em-none-eabihf --release -- -A
    Finished release [optimized + debuginfo] target(s) in 0.02s
led-roulette  :
section              size        addr
.vector_table         256         0x0
.text                6332       0x100
.rodata               648      0x19bc
.data                   0  0x20000000
.bss                 1076  0x20000000
.uninit                 0  0x20000434
.debug_loc           9036         0x0
.debug_abbrev        2754         0x0
.debug_info         96460         0x0
.debug_aranges       1120         0x0
.debug_ranges       11520         0x0
.debug_str          71325         0x0
.debug_pubnames     32316         0x0
.debug_pubtypes     29294         0x0
.ARM.attributes        58         0x0
.debug_frame         2108         0x0
.debug_line         19303         0x0
.comment              109         0x0
Total              283715

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

> **注意**：Cargo项目已经配置为使用LTO构建发布二进制文件。

知道如何读取这个输出吗？该`text`部分包含程序说明。另一方面，`data`和`bss`部分包含静态分配在RAM
中的变量（`static`变量）。如果你还记得你的micro:bit 上的微控制器规格，你应该注意到它的闪存实际上
太小而无法包含这个二进制文件， 那么这怎么可能呢？正如我们从大小统计中看到的那样，大多数二进制
文件实际上是由调试相关部分组成的，但是这些部分不会在任何时候刷新到微控制器， 毕竟它们与执行无关。
