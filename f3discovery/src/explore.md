# 还有什么需要你去探索

我们仅仅触及了表面！还有很多东西留给你去探索。

> **注意：** 如果您正在阅读本文，并且希望帮助在发现书中添加以下任何项目的示例或练习，或任何其他相关嵌入主题，我们很乐意得到您的帮助！
>
> 如果您想提供帮助，但需要帮助或指导如何为本书提供帮助，请[打开一个issue]，或者打开一个添加信息的Pull Request！

[打开一个issue]: https://github.com/rust-embedded/discovery/issues/new

## 关于嵌入式软件的主题

这些主题讨论了编写嵌入式软件的策略。虽然许多问题可以用不同的方式解决，但这些部分讨论了一些策略，以及它们何时有意义（或不意义）。

### 多任务处理

我们所有的程序都执行一个任务。在一个没有操作系统，因此没有线程的系统中，我们如何实现多任务处理。
有两种主要的多任务处理方法：抢先多任务处理和合作多任务处理。

在抢占式多任务处理中，当前正在执行的任务可以在任何时间点被另一个任务*抢占* (中断) 。抢占时，第一个任务将被挂起，
处理器将改为执行第二个任务。 在某个时候，第一个任务将被恢复。微控制器以*中断*的形式为抢占提供硬件支持。

在协作多任务处理中，正在执行的任务将一直运行到*挂起点*。当处理器到达该挂起点时，它将停止执行当前任务，转而执行另一个任务。
在某个时刻，第一个任务将恢复。这两种多任务处理方法之间的主要区别在于，在协作多任务处理中，
在*已知*暂停点*产生*执行控制，而不是在其执行的任何点被强制抢占。

### 睡眠

我们所有的程序都在不断地轮询外围设备，看看是否有什么需要做的。然而，有时什么也做不了！在这些时候，微控制器应该"睡眠"。

当处理器休眠时，它停止执行指令，这节省了电源。节省电源几乎总是一个好主意，因此您的微控制器应该尽可能多地睡眠。
但是，它如何知道何时必须醒来才能执行某些操作？"中断" (见下文了解具体是什么)
是唤醒微控制器的事件之一，但也有其他事件，`wfi`和`wfe`是使处理器"睡眠"的指令。

## 与微控制器功能相关的主题

微控制器（如我们的nRF52/nRF51）具有许多功能。然而，许多共享类似的功能，可用于解决各种不同的问题。

这些主题讨论了其中的一些功能，以及如何在嵌入式开发中有效使用这些功能。

### 直接内存访问（DMA）

该外设是一种*异步*`memcpy`。如果您正在使用micro:bit v2，您实际上已经使用了它，HAL可以通过UART和TWIM外围设备为您完成这项工作。
DMA外围设备可用于执行数据的批量传输。从RAM到RAM，从外设（如UART）到RAM，或从RAM到外设。你可以安排DMA传输，比如
从UART读取256字节到这个缓冲区， 让它在后台运行，然后轮询一些寄存器，看看它是否已经完成，这样你就可以在传输过程中做其他事情。
有关如何实现的更多信息， 请参阅UART一章中的`serial_setup`模块。如果这还不够，您甚至可以尝试深入研究[`nrf52-hal`]的代码。

[`nrf52-hal`]: https://github.com/nrf-rs/nrf-hal

### 中断

为了与现实世界进行交互，微控制器通常需要在某种事件发生时*立即*做出响应。

微控制器具有被中断的能力，这意味着当某个事件发生时，它将停止当前正在执行的任何操作，以响应该事件。
当我们想要在按下按钮时停止电机，或者在计时器完成倒计时时测量传感器时，这非常有用。

虽然这些中断可能非常有用，但它们也可能有点难以正确使用。我们希望确保我们对事件做出快速反应，同时也允许其他工作继续进行。

在Rust中，我们对中断进行建模，类似于桌面Rust程序中的线程概念。这意味着，在主应用程序
和作为处理中断事件的一部分执行的代码之间共享数据时， 我们还必须考虑`Send`和`Sync`的Rust概念。

### 脉宽调制（PWM）

简而言之，PWM是在"接通时间"和"断开时间"之间保持一定比例（"占空比"）的同时，周期性地接通和断开某个部件。
当在具有足够高频率的LED上使用时， 这可用于调暗LED。低占空比，例如10%的开启时间和90%的关闭时间，将使LED非常暗淡，而高占空比
（例如90%的开启和10%的关闭） 将使LED更亮（几乎就像它完全通电一样）。

一般而言，PWM可用于控制向某些电气设备提供多少功率。通过微控制器和电机之间的适当（电力）电子
设备，PWM可用于控制给电机的功率大小， 从而可用于控制其扭矩和速度。然后你可以添加一个角位置传
感器，你得到了一个闭环控制器，可以控制电机在不同负载下的位置。

PWM已经在[`embedded-hal` `Pwm` trait]中抽象出来，您将在[`nrf52-hal`]中再次找到它的实现。

[`embedded-hal` `Pwm` trait]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/trait.Pwm.html

### 数字输入

我们使用微控制器引脚作为数字输出，驱动LED。但是这些引脚也可以被配置为数字输入。
作为数字输入，这些引脚可以读取开关（开/关）或按钮（按下/未按下）的二进制状态。

同样，数字输入在[`embedded-hal` `InputPin` trait]中被抽象，当然[`nrf52-hal`]也有一个实现。

(*阻流板*读取开关/按钮二进制状态并不像听起来那么简单;-) )

[`embedded-hal` `InputPin` trait]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/digital/v2/trait.InputPin.html

### 模数转换器（ADC）

那里有很多数字传感器。您可以使用I2C和SPI等协议来读取它们。但模拟传感器也存在！这些传感器只输出与它们感测的幅度成比例的电压电平。

ADC外围设备可用于将"模拟"电压电平（例如1.25伏）转换为"数字"数字，例如
`[0, 65535]`范围，处理器可在其计算中使用。

同样，[`embedded-hal` `adc` module]以及[`nrf52-hal`]为您提供了支持。

[`embedded-hal` `adc` module]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/adc/index.html

### 数模转换器（DAC）

正如您所预期的，DAC与ADC完全相反。您可以将一些数字值写入寄存器，以在某个"模拟"引脚上产生
`[0, 3.3V]`范围内的电压（假设电源为`3.3V`）。当这个"模拟"引脚连接到一些适当的电子设备，
并且寄存器以恒定的、快速的速率（频率）写入时，使用正确的值，您可以产生声音甚至音乐！

### 实时时钟（RTC）

该外设可用于以"人类格式"跟踪时间。秒、分钟、小时、天、月和年。该外围设备处理从"滴答声"到这些人性化时间单位的转换。
它甚至可以为您处理闰年和夏令时！

### 其他通信协议

- SPI，在[`embedded-hal` `spi` module]中抽象并由[`nrf52-hal`]。
- I2S，目前没有在`embedded-hal`中抽象，但由[`nrf52-hal`]实现。
- 在以太网中，确实存在一个名为[`smoltcp`]的小型TCP/IP堆栈，它是为一些芯片实现的，但micro:bit上的那些芯片没有以太网外设。
- USB，这方面有一些实验工作，例如使用[`usb-device`] crate。
- 蓝牙，确实存在一个不完整的BLE堆栈，名为[`rubble`]，它支持nrf芯片。
- SMBUS，目前既不是在嵌入式`embedded-hal`也不是由[`nrf52-hal`]实现。
- CAN，目前既不是在嵌入式`embedded-hal`也不是由[`nrf52-hal`]实现。
- IrDA，目前既不是在嵌入式`embedded-hal`也不是由[`nrf52-hal`]实现。

[`embedded-hal` `spi` module]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/spi/index.html
[`smoltcp`]: https://github.com/smoltcp-rs/smoltcp
[`usb-device`]: https://github.com/mvirkkunen/usb-device
[`rubble`]: https://github.com/jonas-schievink/rubble

不同的应用程序使用不同的通信协议。面向用户的应用程序通常具有USB连接器，因为USB是PC和智能手机中普遍存在的协议。
而在汽车里，你会发现很多CAN"总线"。一些数字传感器使用SPI，其他使用I2C，其他则使用SMBUS。

如果您碰巧对开发`embedded-hal`的抽象或外围设备的实现感兴趣，请不要害羞地在HAL存储库中打开问题。
或者， 您也可以加入[Rust Embedded matrix channel]，并与大多数从上面构建这些东西的人联系。

## 一般嵌入式相关主题

这些主题涵盖并非特定于我们的设备或设备上的硬件的项目。相反，他们讨论了可用于嵌入式系统的有用技术。

### 陀螺仪

作为我们的Punch-o-meter练习的一部分，我们使用加速度计测量三个维度的加速度变化。
但是还有其他的运动传感器，比如陀螺仪，它可以让我们测量三个维度上"自旋"的变化。

这在尝试构建某些系统时非常有用，例如想要避免翻倒的机器人。
此外，来自陀螺仪等传感器的数据也可以使用一种称为传感器融合的技术与来自加速度计的数据相结合（有关更多信息，请参见下文）。


### 伺服电机和步进电机

虽然一些电机主要用于向一个方向或另一个方向旋转，例如向前或向后驾驶遥控汽车，但有时更精确地测量电机如何旋转是有用的。

我们的微控制器可用于驱动伺服或步进电机，从而可以更精确地控制电机转动的圈数，甚至可以将
电机定位在一个特定的位置，例如，如果我们想移动指向特定方向的时钟。

### 传感器融合

micro:bit含两个运动传感器：加速度计和磁力计。靠它们自己测量：（适当的）加速度和（地球的）磁场。
但是这些量级可以"融合"成更有用的东西：对电路板方向的"稳健"测量。鲁棒意味着比单个传感器具有更少的测量误差。

这种从不同来源获得更可靠数据的想法被称为传感器融合。

---

那么下一步该去哪里呢？有几种选择：

- 您可以查看[`microbit`]板支持箱中的示例。所有这些示例都适用于您拥有的micro:bit板。

[`microbit`]: https://github.com/nrf-rs/microbit/

- 你可以加入[Rust Embedded matrix channel]，那里有很多为嵌入式软件做出贡献或工作的人。
  包括例如编写`microbit` BSP，`nrf52-hal`，`embedded-hal`等的人。

[Rust Embedded matrix channel]: https://matrix.to/#/#rust-embedded:matrix.org

- 如果您正在寻找 Rust Embedded 中可用内容的一般概述，请查看[Awesome Rust Embedded]列表

[Awesome Rust Embedded]: https://github.com/rust-embedded/awesome-embedded-rust/

- 您可以查看[Real-Time Interrupt-driven Concurrency]。一个非常高效的抢占式多任务框架，支持任务优先级和无死锁执行。

[Real-Time Interrupt-driven Concurrency]: https://rtic.rs

- 您可以查看该[`embedded-hal`]项目的更多抽象，甚至可以尝试基于它编写自己的平台无关驱动程序。

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

- 您可以尝试在不同的开发板上运行Rust。最简单的入门方法是使用[`cortex-m-quickstart`]Cargo项目模板。

[`cortex-m-quickstart`]: https://docs.rs/cortex-m-quickstart/0.3.1/cortex_m_quickstart/

- 你可以试试这个[运动传感器演示][madgwick]。有关实现和源代码的详细信息，请参阅[博客文章][wd-1-2]。

[madgwick]: https://mobile.twitter.com/japaricious/status/962770003325005824
[wd-1-2]: http://blog.japaric.io/wd-1-2-l3gd20-lsm303dlhc-madgwick/

- 您可以查看这篇[博客文章][brave-new-io]，它描述了Rust-type系统如何防止I/O配置中的错误。

[brave-new-io]: http://blog.japaric.io/brave-new-io/

- 您可以查看[japaric's blog]，了解有关使用Rust进行嵌入式开发的各种主题。

[japaric's blog]: http://blog.japaric.io


- 您可以加入[Weekly driver initiative] ，帮助我们在`embedded-hal`特性的基础上编写通用驱动程序，适用于各种平台（ARM Cortex-M、AVR、MSP430、RISCV等）

[Weekly driver initiative]: https://github.com/rust-lang-nursery/embedded-wg/issues/39
