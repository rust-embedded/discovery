# 设置开发环境

处理微控制器涉及多种工具，因为我们将处理与您的计算机不同的架构，并且我们必须在"远程"设备上运行和调试程序。

## 文档

不过，工具并不是万能的。没有文档，几乎不可能使用微控制器。

我们将在本书中引用所有这些文档：

- [LSM303AGR]

[LSM303AGR]: https://www.st.com/resource/en/datasheet/lsm303agr.pdf

## 工具

我们将使用下面列出的所有工具。在未指定最低版本的情况下，任何最新版本都可以使用，但我们列出了我们测试过的版本。

- Rust 1.57.0或更新的工具链。

- `gdb-multiarch`。测试版本：10.2。其他版本很可能也可以正常工作，但如果您的发行版/平台不同`gdb-multiarch`用`arm-none-eabi-gdb`，也可以解决问题。
  此外，一些普通gdb的二进制文件也具有多架构功能，您可以在子章节中找到有关此的更多信息。

- [`cargo-binutils`]。版本0.3.3或更高版本。

[`cargo-binutils`]: https://github.com/rust-embedded/cargo-binutils

- [`cargo-embed`]。版本0.18.0或更高版本。

[`cargo-embed`]: https://probe.rs/docs/tools/cargo-embed/

- `minicom`在Linux和macOS上。测试版本：2.7.1。其他版本很可能也能正常工作

- `PuTTY`在Windows上。

接下来，按照与操作系统无关的安装说明获取一些工具：

### `rustc` & Cargo

按照[https://rustup.rs](https://rustup.rs)上的说明安装rustup。

如果您已经安装了rustup，请仔细检查您是否在稳定通道上，并且您的稳定工具链是最新的。
`rustc -V`应该返回一个比下面显示的日期新的日期：

``` console
$ rustc -V
rustc 1.53.0 (53cb7b09b 2021-06-17)
```

### `cargo-binutils`

``` console
$ rustup component add llvm-tools-preview

$ cargo install cargo-binutils --vers 0.3.3

$ cargo size --version
cargo-size 0.3.3
```

### `cargo-embed`

为了安装cargo-embed，首先安装其[先决条件](https://probe.rs/docs/getting-started/installation/), (注意：这些说明是通用的[`probe-rs`](https://probe.rs/)嵌入式调试工具包的一部分). 然后将其与Cargo一起安装：

```console
$ cargo install cargo-embed --vers 0.11.0

$ cargo embed --version
cargo-embed 0.11.0
git commit: crates.io
```

### 这个存储库

由于本书还包含一些在各个章节中使用的小型Rust代码库，因此您还必须下载其源代码。您可以通过以下方式之一执行此操作：

* 访问[存储库](https://github.com/rust-embedded/discovery/)，单击绿色的"Code"按钮，然后单击"Download Zip"下载。
* 使用git从与zip方法中链接的相同存储库中克隆它（如果您知道git，您可能已经安装了它）。

### 操作系统特定说明

现在按照特定于您使用的操作系统的说明进行操作：

- [Linux](linux.md)
- [Windows](windows.md)
- [macOS](macos.md)
