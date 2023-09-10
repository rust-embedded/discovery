# 搭建开发环境

处理微控制器涉及多种工具，因为我们将处理不同于您计算机的体系结构，我们必须在"远程"设备上运行和调试程序。

## 文档

不过，工具并不是万能的。如果没有文档，使用微控制器几乎是不可能的。

我们将在本书中参考所有这些文件：

*说明*：所有这些链接都指向PDF文件，其中一些文件长达数百页，大小为数MB。

- [STM32F3DISCOVERY User Manual][um]
- [STM32F303VC Datasheet][ds]
- [STM32F303VC Reference Manual][rm]
- [LSM303DLHC] \* 
- [L3GD20] \* 

[L3GD20]: https://www.st.com/content/ccc/resource/technical/document/application_note/2c/d9/a7/f8/43/48/48/64/DM00119036.pdf/files/DM00119036.pdf/jcr:content/translations/en.DM00119036.pdf
[LSM303DLHC]: http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf
[ds]: http://www.st.com/resource/en/datasheet/stm32f303vc.pdf
[rm]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf
[um]: http://www.st.com/resource/en/user_manual/dm00063382.pdf

\* **注意**：较新的 (2020/09年左右) Discovery板可能具有不同的电子罗盘和陀螺仪（请参阅用户手册）。
因此，第14-16章中的很多内容将无法正常工作。看看github[issues][gh-issue-274]的问题。

[gh-issue-274]: https://github.com/rust-embedded/discovery/issues/274

## 工具

我们将使用下面列出的所有工具。如果未指定最低版本，则任何最新版本都应有效，但我们已列出了已测试的版本。

- Rust 1.31或更新的工具链。 [USART](../11-usart/index.html)章节要求1.51或更高版本。

- [`itmdump`] >=0.3.1 (`cargo install itm`)。测试版本：0.3.1.

- OpenOCD >=0.8。测试版本：v0.9.0 和 v0.10.0

- `arm-none-eabi-gdb`。强烈建议使用7.12版或更高版本。测试版本：7.10, 7.11, 7.12 和 8.1

- [`cargo-binutils`]. 版本0.1.4或更高。

[`cargo-binutils`]: https://github.com/rust-embedded/cargo-binutils

- Linux和macOS上的`minicom`。测试版本：2.7。 Readers报告`picocom`也可以工作，但本文将使用`minicom`

- Windows上的`PuTTY`。

[`itmdump`]: https://crates.io/crates/itm

如果您的计算机具有蓝牙功能，并且您具有蓝牙模块，则可以额外安装这些工具来使用蓝牙模块。所有这些都是可选的：

- Linux，前提是您没有Blueman这样的蓝牙管理器应用程序。
  - `bluez`
  - `hcitool`
  - `rfcomm`
  - `rfkill`

macOS / OSX / Windows用户只需要其操作系统附带的默认蓝牙管理器。

接下来，按照操作系统不可知的安装说明安装一些工具：

### `rustc` & Cargo

按照说明安装rustup[https://rustup.rs](https://rustup.rs)。

如果您已经安装了rustup，请再次检查您是否处于stable通道中，并且您的stable工具链是最新的。
`rustc -V`返回的日期应比下面显示的日期晚：

``` console
$ rustc -V
rustc 1.31.0 (abe02cefd 2018-12-04)
```

### `itmdump`


``` console
cargo install itm
```

验证版本是否 >=0.3.1
```
$ itmdump -V
itmdump 0.3.1
```

### `cargo-binutils`

安装 `llvm-tools-preview`

``` console
rustup component add llvm-tools-preview
```

安装 `cargo-binutils`
```
cargo install cargo-binutils
```

#### 验证工具是否已安装

在终端上运行以下命令
``` console
cargo new test-size
```
```
cd test-size
```
```
cargo run
```
```
cargo size -- --version
```

结果应该是：
```
~
$ cargo new test-size
     Created binary (application) `test-size` package

~
$ cd test-size

~/test-size (main)
$ cargo run
   Compiling test-size v0.1.0 (~/test-size)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/test-size`
Hello, world!

~/test-size (main)
$ cargo size -- --version
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
LLVM (http://llvm.org/):
  LLVM version 11.0.0-rust-1.50.0-stable
  Optimized build.
  Default target: x86_64-unknown-linux-gnu
  Host CPU: znver2
```

### 操作系统特定指令

现在，按照您使用的操作系统的特定说明进行操作：

- [Linux](linux.md)
- [Windows](windows.md)
- [macOS](macos.md)
