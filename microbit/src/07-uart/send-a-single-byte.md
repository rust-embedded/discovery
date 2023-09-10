# 发送单个字节

我们的第一个任务是通过串行连接从微控制器向计算机发送一个字节。

为了做到这一点，我们将使用以下代码段（这个代码段已经在`07-uart/src/main.rs`中）：

``` rust
{{#include src/main.rs}}
```

这里最流行的新事物显然是`cfg`指令，它有条件地包含/排除部分代码。这主要是因为我们希望使用常规
UART处理micro:bit v1和micro:bit v2。

您还将注意到，这是我们第一次包含一些不是来自库的代码，即`serial_setup`模块。它的唯一目的是为
UARTE提供一个很好的包装器，因此我们可以通过[`embedded_hal::serial`] traits。以与UART完全相同的
方式使用它。 如果您愿意，您可以查看模块的具体功能，但通常不需要理解本章。

[`embedded_hal::serial`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/serial/index.html

除了这些区别之外，UART和UARTE的初始化过程非常相似，所以我们将只讨论UARTE初始化。UARTE使用以下代码进行初始化：
```rs
uarte::Uarte::new(
    board.UARTE0,
    board.uart.into(),
    Parity::EXCLUDED,
    Baudrate::BAUD115200,
);
```
该函数拥有Rust中的UARTE外设表示(`board.UARTE0`)和板上的TX/RX引脚(`board.uart.into()`)，因此，
在使用它们时，其他人不能干扰外置设备或我们的引脚。之后，我们将两个配置选项传递给构造函数：波特率（您应该熟悉）
以及一个名为"对等"的选项。对等校验是一种允许串行通信线路检查其接收的数据在传输过程中是否损坏的方法。 
我们不想在这里使用它，所以我们只是排除它。然后我们将其封装在`UartePort`类型中这样我们就可以以与
micro:bit v1的`serial`相同的方式使用它。

初始化之后，我们通过新创建的uart实例发送`X`。这里的`block!`宏就是`nb::block!`宏。
`nb`是是一个（引用其描述）"最小且可重用的非阻塞I/O层"。它允许我们编写可以在后台执行硬件操作的代码，
同时我们可以进行其他工作（非阻塞）。然而，在这种情况下和其他许多情况下，我们对做其他工作没有兴趣，所以
我们只是调用`block!`其将等待直到I/O操作完成并且已经成功或失败，然后正常继续执行。

最后但并非最不重要的是，我们`flush()`串行端口。这是因为`embedded-hal::serial` traits的实现者可能决定缓冲输出，
直到它接收到一定数量的要发送的字节（UARTE 实现就是这种情况）。调用`flush()`强制它写入当前拥有的字节，而不是等待更多。

## 测试

在闪烁之前，您应该确保启动minicom/PuTTY，因为我们通过串行通信接收的数据没有备份或任何东西，
我们必须实时查看。一旦您的串行监视器启动， 您可以像第5章中所述那样闪存程序：
```
# For micro:bit v2
$ cargo embed --features v2 --target thumbv7em-none-eabihf
  (...)

# For micro:bit v1
$ cargo embed --features v1 --target thumbv6m-none-eabi
```

闪烁结束后，您应该会看到minicom/PuTTY终端上出现字符`X`，恭喜！
