# 闪烁

## Delaying
现在我们将简要介绍一下delay抽象，`embedded-hal`后再将其与上一章中的GPIO抽象结合起来，以最终使LED闪烁。

`embedded-hal`为我们提供了两个抽象来延迟我们程序的执行：[`DelayUs`]和[`DelayMs`]。
除了它们的延迟函数接受不同的单位外，它们基本上都以完全相同的方式工作。

[`DelayUs`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/blocking/delay/trait.DelayUs.html
[`DelayMs`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/blocking/delay/trait.DelayMs.html

在我们的MCU内部，存在几个所谓的"定时器"。他们可以为我们做各种关于时间的事情，包括简单地暂停
我们程序的执行一段固定的时间。 例如，一个非常简单的基于延迟的程序每秒打印一些内容可能如下所示：

```rs
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    loop {
        timer.delay_ms(1000u16);
        rprintln!("1000 ms passed");
    }
}
```

注意，我们在这里将panic实现从`panic_halt`更改为`panic_rtt_target`。这将要求您取消注释`Cargo.toml`
中的两个RTT行，并注释`panic-halt`，因为Rust一次只允许一个panic实现。

为了真正看到打印，我们必须更改`Embed.toml`像这样：
```
[default.general]
# chip = "nrf52833_xxAA" # uncomment this line for micro:bit V2
# chip = "nrf51822_xxAA" # uncomment this line for micro:bit V1

[default.reset]
halt_afterwards = false

[default.rtt]
enabled = true

[default.gdb]
enabled = false
```

现在，在将代码放入`src/main.rs`和另一个`cargo embed` (再次使用之前使用的相同标志)
后您应该会看到"`1000 ms passed`"每秒从MCU发送到控制台。

## 闪烁

现在我们已经到了可以结合我们关于 GPIO 和延迟抽象的新知识的地步，以便真正使micro:bit 背面的LED闪烁。
生成的程序实际上只是上面一个程序和上一节中打开LED的程序的混搭，如下所示：

```rs
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();
    let mut row1 = board.display_pins.row1;

    loop {
        row1.set_low().unwrap();
        rprintln!("Dark!");
        timer.delay_ms(1_000_u16);
        row1.set_high().unwrap();
        rprintln!("Light!");
        timer.delay_ms(1_000_u16);
    }
}
```

在将代码放入`src/main.rs`和最后一个`cargo embed`（带有适当的标志）后，您应该看到我们在闪烁之前点亮的LED以及打印，
每次LED从关闭变为打开，反之亦然。
