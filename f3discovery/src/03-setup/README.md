<!-- # Setting up a development environment -->

# 開発環境の構築

<!-- 
Dealing with microcontrollers involves several tools as we'll be dealing with an architecture
different than your laptop's and we'll have to run and debug programs on a "remote" device.
 -->

開発をするPCとは異なるアーキテクチャを扱うことになるので、マイクロコントローラを扱うには、いくつかのツールが必要です。
私たちは、「リモート」デバイス上でプログラムを実行し、デバッグします。

<!-- ## Documentation -->

## ドキュメント

<!-- 
Tooling is not everything though. Without documentation it is pretty much impossible to work with
microcontrollers.
 -->

ツールだけではありません。ドキュメントがなければ、マイクロコントローラを扱うことはほとんど不可能です。

<!-- We'll be referring to all these documents throughout this book: -->

この本を通して、次のドキュメントを参照します。

<!-- 
*HEADS UP* All these links point to PDF files and some of them are hundreds of pages long and
several MBs in size.
 -->

*注意* これらはPDFファイルへのリンクです。数百ページの長さで数MBもあるファイルもあります。

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

<!-- ## Tools -->

## ツール

<!-- 
We'll use all the tools listed below. Where a minimum version is not specified, any recent version
should work but we have listed the version we have tested.
 -->

下記リストのツールを利用します。最小バージョンが指定されていない場合、新しいバージョンであれば機能するはずです。
私たちがテストしたバージョンをリストに示しています。

<!-- - Rust 1.31 or a newer toolchain. -->

- Rust 1.31以上のツールチェイン

- [`itmdump`] >=0.3.1 (`cargo install itm`). Tested versions: 0.3.1.

<!-- - OpenOCD >=0.8. Tested versions: v0.9.0 and v0.10.0 -->

- OpenOCD >=0.8. テストしたバージョン: v0.9.0 and v0.10.0

<!-- 
- `arm-none-eabi-gdb`. Version 7.12 or newer highly recommended. Tested versions: 7.10, 7.11,
  7.12 and 8.1
 -->

- `arm-none-eabi-gdb`。バージョン7.12以上を強く推奨します。テストしたバージョン: 7.10, 7.11, 7.12, 8.1

<!-- - [`cargo-binutils`]. Version 0.1.4 or newer. -->

- [`cargo-binutils`]. バージョン0.1.4以上

[`cargo-binutils`]: https://github.com/rust-embedded/cargo-binutils

<!-- 
- `minicom` on Linux and macOS. Tested version: 2.7. Readers report that `picocom` also works but
  we'll use `minicom` in this text.
 -->

- LinuxおよびmacOSでは`minicom`。テストしたバージョン: 2.7。読者の報告では`piocom`も動作しますが、このテキストでは`minicom`を使います。

<!-- - `PuTTY` on Windows. -->

- Windoswでは`PuTTY`（訳注：TeraTermでもおそらく大丈夫です）

[`itmdump`]: https://crates.io/crates/itm

<!-- 
If your laptop has Bluetooth functionality and you have the Bluetooth module, you can additionally
install these tools to play with the Bluetooth module. All these are optional:
 -->

あなたのノートPCがBluetooth機能を搭載していて、Bluetoothモジュールを持っている場合、Bluetoothモジュールを使うために、追加で次のツールをインストールして下さい。
これらは、全てオプションです。

<!-- 
- Linux, only if you don't have a Bluetooth manager application like Blueman.
  - `bluez`
  - `hcitool`
  - `rfcomm`
  - `rfkill`
 -->

- Linux。BluemanのようなBluetooth管理アプリケーションがない場合のみ、次のツールをインストールして下さい。
  - `bluez`
  - `hcitool`
  - `rfcomm`
  - `rfkill`

<!-- macOS / OSX / Windows users only need the default bluetooth manager that ships with their OS. -->

macOS / OSX / Windowsユーザーは、OS出荷時のデフォルトBluetoothマネージャだけが必要です。

<!-- Next, follow OS-agnostic installation instructions for a few of the tools: -->

次に、いくつかのOSに依存しないツールのインストール手順を掲載します。

### `rustc` & Cargo

<!-- Install rustup by following the instructions at [https://rustup.rs](https://rustup.rs). -->

[https://rustup.rs](https://rustup.rs)の手順に従って、rustupをインストールします。

<!-- 
If you already have rustup installed double check that you are on the stable
channel and your stable toolchain is up to date. `rustc -V` should return a date
newer than the one shown below:
 -->

既にrustupをインストールしてある場合、stableチャネルになっていて、stableツールチェインが最新であることを確認して下さい。
`rustc -V`は、下よりも新しい日付を返す必要が有ります。

``` console
$ rustc -V
rustc 1.31.0 (abe02cefd 2018-12-04)
```

### `itmdump`


``` console
cargo install itm
```

Verify the version is >=0.3.1
```
$ itmdump -V
itmdump 0.3.1
```

### `cargo-binutils`

Install `llvm-tools-preview`

``` console
rustup component add llvm-tools-preview
```

Install `cargo-binutils`
```
cargo install cargo-binutils
```

#### Verify tools are installed

Run the following commands at your terminal
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

The results should be something like:
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

<!-- ### OS specific instructions -->

### OS固有の手順

<!-- Now follow the instructions specific to the OS you are using: -->

使用しているOSに固有の手順に従って下さい。

- [Linux](linux.md)
- [Windows](windows.md)
- [macOS](macos.md)
