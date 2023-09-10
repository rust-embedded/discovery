# 挑战

你现在已经武装好迎接挑战了！你的任务是实现我在本章开头向你展示的应用程序。

<p>
<video src="../assets/roulette_fast.mp4" loop autoplay>
</p>

如果您不能确切地看到这里发生了什么，那么它的版本要慢得多：

<p>
<video src="../assets/roulette_slow.mp4" loop autoplay>
</p>

由于单独使用 LED 引脚非常烦人（特别是如果您必须像这里一样使用基本上所有这些引脚），
您可以使用BSP提供的显示API。它是这样工作的：

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let light_it_all = [
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
    ];

    loop {
        // Show light_it_all for 1000ms
        display.show(&mut timer, light_it_all, 1000);
        // clear the display again
        display.clear();
        timer.delay_ms(1000_u32);
    }
}
```

有了这个API，您的任务基本上归结为只需计算适当的图像矩阵并将其传递到BSP。
