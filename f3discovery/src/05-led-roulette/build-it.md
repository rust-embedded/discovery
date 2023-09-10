# 构建

第一步是构建我们的"二进制" crate。因为微控制器的架构与您的计算机不同，所以我们必须进行交叉编译。
Rust land中的交叉编译就像向`rustc`或Cargo传递一个额外的`--target`标志一样简单。
复杂的部分是找出该标志的参数：target的*名字*。

F3中的微控制器中有一个Cortex-M4F处理器。`rustc`知道如何交叉编译到Cortex-M架构，并
提供4个不同的targets，涵盖该架构中的不同处理器系列：

- `thumbv6m-none-eabi`, 适用于Cortex-M0 和 Cortex-M1处理器
- `thumbv7m-none-eabi`, 适用于Cortex-M3处理器
- `thumbv7em-none-eabi`, 适用于Cortex-M4 和 Cortex-M7处理器
- `thumbv7em-none-eabihf`, 用于Cortex-M4**F** 和 Cortex-M7**F**处理器

对于F3，我们将使用`thumbv7em-none-eabihf`。在交叉编译之前，您必须为目标下载标准库的预编译版本
（实际上是标准库的缩减版本）。这是使用`rustup`完成的：

``` console
rustup target add thumbv7em-none-eabihf
```

您只需执行上述步骤一次；每当您更新工具链时，`rustup`都会重新安装一个新的标准库(`rust-std`组件) 。

有了`rust-std`组件，您现在可以使用Cargo交叉编译程序。

> **注意**：确保您位于`src/05-led-roulette`目录中，并运行下面的`cargo build`命令以创建可执行文件：
``` console
cargo build --target thumbv7em-none-eabihf
```
在控制台上，您应该看到以下内容：
``` console
$ cargo build --target thumbv7em-none-eabihf
   Compiling typenum v1.12.0
   Compiling semver-parser v0.7.0
   Compiling version_check v0.9.2
   Compiling nb v1.0.0
   Compiling void v1.0.2
   Compiling autocfg v1.0.1
   Compiling cortex-m v0.7.1
   Compiling proc-macro2 v1.0.24
   Compiling vcell v0.1.3
   Compiling unicode-xid v0.2.1
   Compiling stable_deref_trait v1.2.0
   Compiling syn v1.0.60
   Compiling bitfield v0.13.2
   Compiling cortex-m v0.6.7
   Compiling cortex-m-rt v0.6.13
   Compiling r0 v0.2.2
   Compiling stm32-usbd v0.5.1
   Compiling stm32f3 v0.12.1
   Compiling usb-device v0.2.7
   Compiling cfg-if v1.0.0
   Compiling paste v1.0.4
   Compiling stm32f3-discovery v0.6.0
   Compiling embedded-dma v0.1.2
   Compiling volatile-register v0.2.0
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling semver v0.9.0
   Compiling generic-array v0.14.4
   Compiling switch-hal v0.3.2
   Compiling num-traits v0.2.14
   Compiling num-integer v0.1.44
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling cast v0.2.3
   Compiling quote v1.0.9
   Compiling generic-array v0.13.2
   Compiling generic-array v0.12.3
   Compiling generic-array v0.11.1
   Compiling panic-itm v0.4.2
   Compiling lsm303dlhc v0.2.0
   Compiling as-slice v0.1.4
   Compiling micromath v1.1.0
   Compiling accelerometer v0.12.0
   Compiling chrono v0.4.19
   Compiling aligned v0.3.4
   Compiling rtcc v0.2.0
   Compiling cortex-m-rt-macros v0.1.8
   Compiling stm32f3xx-hal v0.6.1
   Compiling aux5 v0.2.0 (~/embedded-discovery/src/05-led-roulette/auxiliary)
   Compiling led-roulette v0.2.0 (~/embedded-discovery/src/05-led-roulette)
    Finished dev [unoptimized + debuginfo] target(s) in 17.91s
```

> **注意**：确保编译此crate时*不进行*优化。提供的Cargo.toml文件和build命令将确保优化关闭。

好了，现在我们已经生成了一个可执行文件。这个可执行文件不会闪烁任何LED，它只是一个简化的版本，我们将在本章稍后部分进行构建。
作为健全性检查，让我们验证生成的可执行文件实际上是ARM二进制文件：

``` console
cargo readobj --target thumbv7em-none-eabihf --bin led-roulette -- --file-header
```
`cargo readobj ..`相当于`readelf -h target/thumbv7em-none-eabihf/debug/led-roulette`应该产生类似的结果：
``` console
$ cargo readobj --target thumbv7em-none-eabihf --bin led-roulette -- --file-header
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x8000195
  Start of program headers:          52 (bytes into file)
  Start of section headers:          818328 (bytes into file)
  Flags:                             0x5000400
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         4
  Size of section headers:           40 (bytes)
  Number of section headers:         22
  Section header string table index: 20
  ```

接下来，我们将把程序闪存到微控制器中。
