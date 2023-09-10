# 点亮
## embedded-hal

在本章中，我们将点亮micro:bit背面的众多LED中的一个，因为这基本上是嵌入式编程的"Hello World"。
为了完成这项任务，我们将使用提供的特性之一`embedded-hal`，特别是[`OutputPin`]允许我们打开或关闭引脚的特性。

[`OutputPin`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/digital/v2/trait.OutputPin.html

## micro:bit LEDs

在micro:bit的背面，您可以看到一个5x5方形的LED，通常称为LED矩阵。使用这种矩阵对齐方式，我们不
必使用25个单独的引脚来驱动每个LED， 而只需使用10(5+5)个引脚来控制矩阵的哪一列和哪一行点亮。

> **注意**：micro:bit v1团队的实现方式略有不同。他们的[原理图页面]说它实际上是作为3x9矩阵实现的，但有几列根本没有使用。

通常，为了确定我们必须控制哪些特定引脚以点亮特定 LED，我们现在必须分别读取[micro:bit v2 原理图]或[micro:bit v1 原理图]。
幸运的是，我们可以使用前面提到的micro:bit BSP，它将所有这些都很好地抽象出来。

[原理图页面]: https://tech.microbit.org/hardware/schematic/
[micro:bit v2 原理图]: https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.00/MicroBit_V2.0.0_S_schematic.PDF
[micro:bit v1 原理图]: https://github.com/bbcmicrobit/hardware/blob/master/V1.5/SCH_BBC-Microbit_V1.5.PDF

## 居然亮了！

点亮矩阵中的LED所需的代码实际上非常简单，但需要一些设置。首先看一下，然后我们可以一步一步地进行：

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::board::Board;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    loop {}
}
```

main函数的前几行只是做一些我们之前已经看过的基本导入和设置。但是，main函数看起来与我们现在看到的完全不同。

第一行与大多数用Rust编写的HAL在内部如何工作有关。如前所述，它们建立在拥有（在Rust意义上）芯片的所有外围设备的PAC Crate之上。 
`let mut board = Board::take().unwrap();`基本上从PAC中获取所有这些外围设备并将它们绑定到一个变量。
在这种特定情况下，我们不仅使用HAL，而且使用整个BSP，因此这也获得了板上其他芯片的 Rust 表示的所有权。

> **注意**：如果您想知道为什么我们必须在这里调用`unwrap()`，理论上可以多次调用`take()`这将导致
> 外围设备由两个单独的变量表示，因此会出现许多可能的混淆行为，因为两个变量修改相同的资源。
> 为了避免这种情况，PAC的实现方式是， 如果您两次尝试使用外围设备，它会出现panic。

现在，我们可以通过将`row1`引脚设置为高（即打开）来点亮连接到`row1`，`col1`的LED。
我们可以将`col1`设置为低的原因是因为LED矩阵电路的工作方式。此外，`embedded-hal`的设计方式是，硬件
上的每个操作都可能返回错误，即使只是打开或关闭引脚。因为这在我们的情况下是极不可能的，所以我们可以只`unwrap()`结果。

## 测试

测试我们的小程序非常简单。先把它放到`src/main.rs`。然后，我们只需再次运行最后一节中的
`cargo embed`命令，让它像以前一样闪烁。然后打开我们的GDB并连接到GDB stub:

```
$ # Your GDB debug command from the last section
(gdb) target remote :1337
Remote debugging using :1337
cortex_m_rt::Reset () at /home/nix/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.12/src/lib.rs:489
489     pub unsafe extern "C" fn Reset() -> ! {
(gdb)
```

如果我们现在让程序通过GDB`continue`命令运行，micro:bit背面的LED之一应该会亮起。
