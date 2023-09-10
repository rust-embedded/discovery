# Rust Embedded 术语
在深入研究micro:bit编程之前，让我们快速浏览一下对未来所有章节都非常重要的库和术语。

## 抽象层（Abstraction layers）
对于任何完全受支持的微控制器/带有微控制器的板，您通常会听到以下术语用于它们的抽象级别：

### 外设访问箱 (PAC)
PAC的工作是为芯片的外围设备提供一个安全的(ish)直接接口，允许您根据需要配置每一个最后一位（当然也以错误的方式）。
通常，如果更高的层不能满足您的需求或者您正在开发它们时，您只需要处理PAC。我们（毫无疑问）要使用的PAC是用于[nRF52]
或用于[nRF51]的PAC。

### 硬件抽象层 (HAL)
HAL的工作是在芯片的PAC之上构建，并提供一个抽象，该抽象实际上可用于不了解该芯片所有特殊行为的人。
通常，它们将整个外围设备抽象为单个结构，例如，这些结构可用于通过外围设备发送数据。
我们将分别使用[nRF52-hal]或[nRF51-hal]。

### Board Support Crate (历史上称为 Board Support Package, 或 BSP)
BSP的工作是一次性抽象出整个板子 (例如micro:bit) 。这意味着它必须提供抽象来使用微控制器以及板上可能存在的传感器、LED等。
很多时候（尤其是使用定制板），您将只使用芯片的HAL并自己构建传感器的驱动程序或在crates.io上搜索它们。不过对我们来说幸运的是，
micro:bit确实有一个[BSP]，所以我们也将在HAL之上使用它。

[nrF52]: https://crates.io/crates/nrf52833-pac
[nrF51]: https://crates.io/crates/nrf51
[nrF52-hal]: https://crates.io/crates/nrf52833-hal
[nrF51-hal]: https://crates.io/crates/nrf51-hal
[BSP]: https://crates.io/crates/microbit

## 统一图层（Unifying the layers）

接下来我们将看看Rust Embedded world中一个非常核心的软件：[`embedded-hal`]。顾名思义，它与我们
了解的第二层抽象有关：HALs。背后的想法[`embedded-hal`]是提供一组描述行为的特征，这些特征通常在
所有 HAL 中特定外围设备的所有实现中共享。例如，人们总是希望具有能够打开或关闭引脚电源的功能。
例如打开和关闭板上的 LED。 这允许我们为温度传感器编写驱动程序，该驱动程序可以在任何[`embedded-hal`]
存在特征实现的芯片上使用， 只需以仅依赖于[`embedded-hal`]traits。以这种方式编写的驱动程序被称
为平台无关， 幸运的是，crates.io上的大多数驱动程序实际上都是平台无关的。

[`embedded-hal`]: https://crates.io/crates/embedded-hal


## 进一步阅读

如果您想了解有关这些抽象级别的更多信息，Franz Skarman（又名[TheZoq2]）在Oxidize 2020期间举行
了一次关于这个主题的演讲，名为[An Overview of the Embedded Rust Ecosystem]。

[TheZoq2]: https://github.com/TheZoq2/
[An Overview of the Embedded Rust Ecosystem]: https://www.youtube.com/watch?v=vLYit_HHPaY
