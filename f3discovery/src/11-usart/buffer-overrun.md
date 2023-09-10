# 溢出

如果你这样编写程序：

``` rust
{{#include examples/buffer-overrun.rs}}
```

当您在调试模式下执行编译的程序时，可能会在计算机上收到类似的消息。

``` console
$ # minicom's terminal
(..)
The uic brwn oxjums oer helaz do.
```

如果你在发布模式下编译，你可能只得到这样的结果：

``` console
$ # minicom's terminal
(..)
T
```

出了什么问题？

你看，通过网络发送字节需要相当长的时间。我已经做了数学，所以让我引用自己的话：

> 理论上，使用1个起始bit，8个数据bits，1个stop bit和115200 bps波特率的通用配置，可以每秒发送11520帧。
> 由于每一帧携带一字节数据，因此数据速率为11.52 KB/s

我们的全字母短句长度为45字节。这意味着发送字符串至少需要3900微秒(`45 bytes / (11,520 bytes/s) = 3,906 us`) 。
处理器的工作频率为8 MHz，执行一条指令需要125纳秒，所以很可能在3900微秒以内完成`for`循环。

我们实际上可以计算执行`for`循环环所需的时间。`aux11::init()`返回一个`MonoTimer` (单调计时器) 该值公开一个与
`std::time`中的API类似`Instant`API。

``` rust
{{#include examples/buffer-overrun-timed.rs}}
```

在调试模式下，我得到：

``` console
$ # itmdump terminal
(..)
`for` loop took 22415 ticks (2801.875 us)
```

这不到3900微秒，但并不遥远，这就是为什么只有几个字节的信息丢失的原因。

总之，处理器试图以比硬件实际处理速度更快的速度发送字节，这会导致数据丢失。这种情况称为缓冲区*溢出*。

我们如何避免这种情况？状态寄存器 (`ISR`) 有一个标志`TXE`，指示在不导致数据丢失的情况下写入
`TDR`寄存器是否"安全"

让我们用它来降低处理器的速度。

``` rust
{{#include examples/buffer-overrun-txe.rs}}
```

这次，在调试或发布模式下运行程序应该会在接收端产生一个完整的字符串。

``` console
$ # minicom/PuTTY's console
(..)
The quick brown fox jumps over the lazy dog.
```

`for`循环的计时也应该接近理论上的3900微秒。以下是调试版本的计时。

``` console
$ # itmdump terminal
(..)
`for` loop took 30499 ticks (3812.375 us)
```
