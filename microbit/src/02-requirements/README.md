# 硬件 / 知识要求

阅读本书的主要知识要求是了解*一些*Rust。我们很难量化*一些*，但至少我可以告诉你，你不需要完全理解泛型，但你需要知道如何*使用*闭包。
您还需要熟悉[2018 版]的语法，`extern crate`尤其是在2018版中不需要的事实。

[2018 版]: https://rust-lang-nursery.github.io/edition-guide/

此外，要遵循本材料，您需要以下硬件：

- 一个[micro:bit v2]板，或者一个[micro:bit v1.5]板，本书将v1.5称为v1。

[micro:bit v2]: https://tech.microbit.org/hardware/
[micro:bit v1.5]: https://tech.microbit.org/hardware/1-5-revision/

(您可以从多家[电子][0] [供应商][1])处购买此板.

[0]: https://microbit.org/buy/
[1]: https://www.mouser.com/microbit/_/N-aez3t?P=1y8um0l

<p align="center">
<img title="micro:bit" src="../assets/microbit-v2.jpg">
</p>

> **注意** 这是micro:bit v2的图像，v1的正面看起来略有不同

- 一根micro-B USB电缆，需要使micro:bit板工作。确保数据线支持数据传输，因为某些数据线仅支持充电设备。

<p align="center">
<img title="micro-B USB cable" src="../assets/usb-cable.jpg">
</p>

> **注意** 您可能已经拥有这样的电缆，因为某些micro:bit套件随附此类电缆。
> 一些用于为移动设备充电的USB电缆也可以工作，
> 如果它们是micro-B并且具有传输数据的能力。

> **常见问题解答**：等等，为什么我需要这个特定的硬件？

它让我和你的生活更轻松。

如果我们不必担心硬件差异，那么材料就更容易接近了。相信我这个。

> **常见问题解答**：我可以使用不同的开发板遵循此材料吗？

也许？这主要取决于两件事：您以前使用微控制器的经验和/或是否已经存在高级板条箱，例如[`nrf52-hal`]，您的开发板的某个地方。
如果您打算使用其他微控制器，可以查看[Awesome Embedded Rust HAL list]。

[`nrf52-hal`]: https://docs.rs/nrf52-hal
[Awesome Embedded Rust HAL list]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates

使用不同的开发板，本文将失去大部分（如果不是全部）初学者友好性和"易于理解"，IMO。

如果您有不同的开发板并且您不认为自己完全是初学者，那么您最好从快速入门项目模板开始。

[快速入门]: https://rust-embedded.github.io/cortex-m-quickstart/cortex_m_quickstart/
