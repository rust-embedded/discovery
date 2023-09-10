# 读取单个寄存器

让我们将所有理论付诸实践！

首先，我们需要知道芯片内的加速度计和磁力计的从地址，这些可以在第39页的LSM303AGR数据表中找到，它们是：

- 0011001 for the accelerometer
- 0011110 for the magnetometer

> **注意**：请记住，这些只是地址的前7位，第8位将是决定我们是执行读取还是写入的位。

接下来我们需要一个寄存器来读取。许多I2C芯片将提供某种设备标识寄存器，供其主机读取。这样做是因
为考虑到成千上万（甚至数百万）的I2C芯片， 很可能在某一时刻，两个具有相同地址的芯片最终将被构建
（毕竟地址"仅"7位宽）。有了这个设备ID寄存器，驱动程序可以确保它确实在与LSM303AGR通信，而不是
与恰好具有相同地址的其他芯片通信。正如您可以在 LSM303AGR 的数据表（特别是第46页和第61页）
中阅读的那样，它确实提供了两个寄存器， 分别称为`WHO_AM_I_A`地址`0x0f`和`WHO_AM_I_M`地址`0x4f`
其中包含一些设备独有的位模式（A与加速度计相同，M与磁力计相同）。

现在唯一缺少的是软件部分，即`microbit`我们应该为此使用/the HAL crates的哪个API。但是，如果您仔
细阅读您正在使用的nRF芯片的数据表， 您很快就会发现它们实际上并没有I2C外设。不过对我们来说幸运的是，它们
有与I2C兼容的TWI（双线接口）和TWIM（取决于您使用的芯片，就像UART和UART一样）。

现在，如果我们将`microbit` crate中[`twi(m)`模块]的文档与我们迄今为止收集到的所有其他信息放在一起，
我们将得到这段代码来读取和打印两个设备ID：

[`twi(m)`模块]: https://docs.rs/microbit-v2/0.11.0/microbit/hal/twim/index.html

``` rust
{{#include src/main.rs}}
```

除了初始化之外，如果您理解前面描述的I2C协议，那么这段代码应该是直截了当的。这里的初始化与UART章节中的初始化类似。
我们将外围设备以及用于与芯片通信的引脚传递给构造器；然后是我们希望总线工作的频率，在这种情况下为100kHz（`K100`）。

## 测试
与往常一样，您必须修改`Embed.toml`以适合您的MCU，然后可以使用：
```console
# For micro:bit v2
$ cargo embed --features v2 --target thumbv7em-none-eabihf

# For micro:bit v1
$ cargo embed --features v1 --target thumbv6m-none-eabi
```
为了测试我们的小示例程序。
