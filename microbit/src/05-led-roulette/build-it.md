# 构建

第一步是构建我们的"binary" crate。因为微控制器的架构与您的计算机不同，所以我们必须交叉编译。
在Rust领域进行交叉编译就像向`rustc`或Cargo传递一个额外的`--target`标志一样简单。
复杂的部分是弄清楚这个标志的参数：目标的*名称*。

我们已经知道micro:bit v2上的微控制器内部有一个Cortex-M4F处理器，v1上的那个是Cortex-M0。
`rustc`知道如何交叉编译到Cortex-M架构，并提供了几个不同的目标，涵盖了该架构中的不同处理器系列：

- `thumbv6m-none-eabi`，适用于Cortex-M0和Cortex-M1处理器
- `thumbv7m-none-eabi`，适用于Cortex-M3处理器
- `thumbv7em-none-eabi`，适用于Cortex-M4和Cortex-M处理器
- `thumbv7em-none-eabihf`，适用于Cortex-M4**F**和Cortex-M7**F**处理器
- `thumbv8m.main-none-eabi`，适用于Cortex-M33和Cortex-M35P处理器
- `thumbv8m.main-none-eabihf`，适用于Cortex-M33**F**和Cortex-M35P**F**处理器

对于micro:bit v2，我们将使用`thumbv7em-none-eabihf`target，对于v1，我们将使用`thumbv6m-none-eabi`target。
在交叉编译之前，您必须为您的目标下载标准库的预编译版本（实际上是它的简化版本）。这是使用`rustup`:

``` console
# For micro:bit v2
$ rustup target add thumbv7em-none-eabihf
# For micro:bit v1
$ rustup target add thumbv6m-none-eabi
```

您只需执行上述步骤一次；每当您更新工具链时，`rustup`都会重新安装新的(`rust-std`组件)。
因此如果您在[验证步骤]时已经添加了必要的target，则可以跳过此步骤。

[验证步骤]: ../03-setup/verify.html#verifying-cargo-embed


有了`rust-std`组件，您现在可以使用Cargo交叉编译程序：

``` console
# make sure you are in the `src/05-led-roulette` directory

# For micro:bit v2
$ cargo build --features v2 --target thumbv7em-none-eabihf
   Compiling semver-parser v0.7.0
   Compiling typenum v1.12.0
   Compiling cortex-m v0.6.3
   (...)
   Compiling microbit-v2 v0.10.1
    Finished dev [unoptimized + debuginfo] target(s) in 33.67s

# For micro:bit v1
$ cargo build --features v1 --target thumbv6m-none-eabi
   Compiling fixed v1.2.0
   Compiling syn v1.0.39
   Compiling cortex-m v0.6.3
   (...)
   Compiling microbit v0.10.1
	Finished dev [unoptimized + debuginfo] target(s) in 22.73s
```

> **注意**：一定要编译这个crate*而不进行*优化。上面提供的Cargo.toml文件和构建命令将确保优化关闭。

好的，现在我们已经生成了一个可执行文件。这个可执行文件不会闪烁任何LED，它只是一个简化版本，我
们将在本章后面进行构建。作为健全性检查，让我们验证生成的可执行文件实际上是ARM二进制文件：

``` console
# For micro:bit v2
# equivalent to `readelf -h target/thumbv7em-none-eabihf/debug/led-roulette`
$ cargo readobj --features v2 --target thumbv7em-none-eabihf --bin led-roulette -- --file-headers
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
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
  Entry point address:               0x117
  Start of program headers:          52 (bytes into file)
  Start of section headers:          793112 (bytes into file)
  Flags:                             0x5000400
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         4
  Size of section headers:           40 (bytes)
  Number of section headers:         21
  Section header string table index: 19

# For micro:bit v1
# equivalent to `readelf -h target/thumbv6m-none-eabi/debug/led-roulette`
$ cargo readobj --features v1 --target thumbv6m-none-eabi --bin led-roulette -- --file-headers
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
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
  Entry point address:               0xC1
  Start of program headers:          52 (bytes into file)
  Start of section headers:          693196 (bytes into file)
  Flags:                             0x5000200
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         4
  Size of section headers:           40 (bytes)
  Number of section headers:         22
  Section header string table index: 20
```

接下来，我们将把程序嵌入到我们的微控制器中。
