# 接收单个字节

到目前为止，我们可以将数据从微控制器发送到您的计算机。是时候尝试相反的方法了：从计算机接收数据。
幸运的是，`embedded-hal`再次让我们了解了这一点：

``` rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    loop {
        let byte = nb::block!(serial.read()).unwrap();
        rprintln!("{}", byte);
    }
}
```

与发送字节程序相比，唯一改变的部分是`main()`末尾的循环。在这里，我们使用`embedded-hal`提供的`read()`函数，
以等待一个字节可用并读取它。然后，我们将该字节打印到RTT调试控制台中，以查看这些东西是否实际到达。

请注意，如果您刷新此程序并开始在`minicom`中键入字符以将其发送给微控制器，您将只能在RTT控制台
中看到数字，因为我们没有将收到的`u8`转换为实际`char`。由于从`u8`到`char`的转换非常简单，如果您真
的想看到RTT控制台中的字符，我将把这个任务留给您。
