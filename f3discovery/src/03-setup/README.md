# Setting up a development environment

Dealing with microcontrollers involves several tools as we'll be dealing with an architecture
different than your computer's and we'll have to run and debug programs on a "remote" device.

## Documentation

Tooling is not everything though. Without documentation it is pretty much impossible to work with
microcontrollers.

We'll be referring to all these documents throughout this book:

*HEADS UP* All these links point to PDF files and some of them are hundreds of pages long and
several MBs in size.

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

\* **NOTE**: Newer (from around 2020/09) Discovery boards may have a different e-compass and gyroscope (see the user manual). 
As such, much in chapters 14-16 will not work as is. 
Checkout the github issues like [this][gh-issue-274]. 

[gh-issue-274]: https://github.com/rust-embedded/discovery/issues/274

## Tools

We'll use all the tools listed below. Where a minimum version is not specified, any recent version
should work but we have listed the version we have tested.

- Rust 1.31 or a newer toolchain. Chapter [USART](../11-usart/index.html)
  requires 1.51 or newer.

- [`itmdump`] >=0.3.1 (`cargo install itm`). Tested versions: 0.3.1.

- OpenOCD >=0.8. Tested versions: v0.9.0 and v0.10.0

- `arm-none-eabi-gdb`. Version 7.12 or newer highly recommended. Tested versions: 7.10, 7.11,
  7.12 and 8.1

- [`cargo-binutils`]. Version 0.1.4 or newer.

[`cargo-binutils`]: https://github.com/rust-embedded/cargo-binutils

- `minicom` on Linux and macOS. Tested version: 2.7. Readers report that `picocom` also works but
  we'll use `minicom` in this text.

- `PuTTY` on Windows.

[`itmdump`]: https://crates.io/crates/itm

If your computer has Bluetooth functionality and you have the Bluetooth module, you can additionally
install these tools to play with the Bluetooth module. All these are optional:

- Linux, only if you don't have a Bluetooth manager application like Blueman.
  - `bluez`
  - `hcitool`
  - `rfcomm`
  - `rfkill`

macOS / OSX / Windows users only need the default bluetooth manager that ships with their OS.

Next, follow OS-agnostic installation instructions for a few of the tools:

### `rustc` & Cargo

Install rustup by following the instructions at [https://rustup.rs](https://rustup.rs).

If you already have rustup installed double check that you are on the stable
channel and your stable toolchain is up to date. `rustc -V` should return a date
newer than the one shown below:

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

Install `llvm-tools`

``` console
rustup component add llvm-tools
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

### OS specific instructions

Now follow the instructions specific to the OS you are using:

- [Linux](linux.md)
- [Windows](windows.md)
- [macOS](macos.md)
