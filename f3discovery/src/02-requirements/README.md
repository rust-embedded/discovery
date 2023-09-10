# 硬件/知识要求

阅读本书的主要知识要求是了解*一些*Rust。我们很难量化一些，但至少我可以告诉你，你不需要完全理解
泛型， 但你需要知道如何*使用*闭包。您还需要熟悉[2018版]的语法，`extern crate`尤其是在2018版中不需要的事实。

[2018版]: https://rust-lang-nursery.github.io/edition-guide/

由于嵌入式编程的性质，了解二进制和十六进制值的表示方式以及一些按位运算符的使用也将非常有帮助。
例如，了解以下程序是如何产生输出的，这将很有用。

```rust
fn main() {
    let a = 0x4000_0000 + 0xa2;

    // Use of the bit shift "<<" operation.
    let b = 1 << 5;

    // {:X} will format values as hexadecimal
    println!("{:X}: {:X}", a, b);
}
```

此外，要遵循此材料，您需要以下硬件：

(某些组件是可选的，但建议使用)

- 一个[STM32F3DISCOVERY]开发板。

[STM32F3DISCOVERY]: http://www.st.com/en/evaluation-tools/stm32f3discovery.html

(您可以从"大型"[电子产品][0] [供应商][1]或[电子商务][2][网站][3]购买此开发板)

[0]: http://www.mouser.com/ProductDetail/STMicroelectronics/STM32F3DISCOVERY
[1]: http://www.digikey.com/product-detail/en/stmicroelectronics/STM32F3DISCOVERY/497-13192-ND
[2]: https://www.aliexpress.com/wholesale?SearchText=stm32f3discovery
[3]: http://www.ebay.com/sch/i.html?_nkw=stm32f3discovery

<p>
<img title="STM32F3DISCOVERY" src="../assets/f3.jpg">
</p>

- 可选。 **3.3V** USB <-> 串行模块。详细说明：如果您有发现板的最新版本之一
  （鉴于第一个版本是几年前发布的，通常是这种情况），那么您*不*需要此模块， 因为开发板包含此功能。如果您有较旧版本
  的电路板，则需要在第10章和第11章中使用此模块。为完整起见，我们将提供使用串行模块的说明。
  这本书将使用[这种特定的模式][sparkfun]，但你可以使用任何其他模式，只要它在3.3V下运行。您可以从
  [电子商务][4]网站购买的CH340G模块也可以使用，而且可能更便宜。

[sparkfun]: https://www.sparkfun.com/products/9873
[4]: https://www.aliexpress.com/wholesale?SearchText=CH340G

<p>
<img title="A 3.3v USB <-> Serial module" src="../assets/serial.jpg">
</p>

- 可选。HC-05蓝牙模块（带标头！）。HC-06也会起作用。

(与其他中国零件一样，你几乎只能在[电子商务][5] [网站][6]上找到这些零件。（美国）电子产品供应商通常出于某种原因不库存这些零件)

[5]: http://www.ebay.com/sch/i.html?_nkw=hc-05
[6]: https://www.aliexpress.com/wholesale?SearchText=hc-05

<p>
<img title="The HC-05 Bluetooth module" src="../assets/bluetooth.jpg">
</p>

- 两条mini-B USB电缆。STM32F3DISCOVERY板工作需要一个。另一个仅当您具有串行<-> USB模块时才需要。
  确保两条电缆都支持数据传输，因为某些电缆仅支持充电设备。

<p>
<img title="mini-B USB cable" src="../assets/usb-cable.jpg">
</p>

> **注意**：这些**不是**几乎所有Android手机都附带的USB电缆；这些是
> *微型*USB电缆。确保你有正确的东西！

- 大部分是可选的。 5根母对母、4根公对母和1根公对公*跳线* (AKA Dupont)。
  你*很可能*需要一个母对母线来让ITM工作。只有当您使用USB<-> 串行和蓝牙模块时，才需要其他导线。

(您可以从电子[电子供应商][7]或[电子商务][8] [网站][9]获取这些信息)

[7]: https://www.adafruit.com/categories/306
[8]: http://www.ebay.com/sch/i.html?_nkw=dupont+wire
[9]: https://www.aliexpress.com/wholesale?SearchText=dupont+wire

<p>
<img title="Jumper wires" src="../assets/jumper-wires.jpg">
</p>

> **FAQ**：等等，我为什么需要这个特定的硬件？

这让我和你的生活变得更加轻松。

如果我们不必担心硬件差异，那么这种材料就更容易接近。相信我。

> **FAQ**：我可以用不同的开发板来学习这些材料吗？

大概这主要取决于两件事：您以前使用微控制器的经验和/或是否已经在某个地方为您的开发板提供了像[`f3`]这样的高级crate。

[`f3`]: https://docs.rs/f3

如果使用不同的开发板，这篇文章将失去大部分（如果不是全部的话）初学者友好性和"易于理解"性。

如果你有一个不同的开发板，并且你不认为自己完全是初学者，那么最好从[快速启动]项目模板开始。

[快速启动]: https://rust-embedded.github.io/cortex-m-quickstart/cortex_m_quickstart/
