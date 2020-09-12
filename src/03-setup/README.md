# Setting up a development environment

Dealing with microcontrollers involves several tools as we'll be dealing with an architecture
different than your computer's and we'll have to run and debug programs on a "remote" device.

## Documentation

Tooling is not everything though. Without documentation it is pretty much impossible to work with
microcontrollers.

We'll be referring to all these documents throughout this book:

- [LSM303AGR]

[LSM303AGR]: https://www.st.com/resource/en/datasheet/lsm303agr.pdf

## Tools

We'll use all the tools listed below. Where a minimum version is not specified, any recent version
should work but we have listed the version we have tested.

- Rust 1.45.2 or a newer toolchain.

- `gdb-multiarch`. Tested version: 9.1. Other versions will most likely work as well though
  If your distribution/platform does not have `gdb-multiarch` available `arm-none-eabi-gdb`
  will do the trick as well.

- [`cargo-binutils`]. Version 0.1.4 or newer.

[`cargo-binutils`]: https://github.com/rust-embedded/cargo-binutils

- [`cargo-embed`]. Version 0.9.0 or newer.

[`cargo-embed`]: https://github.com/probe-rs/cargo-embed

- `minicom` on Linux and macOS. Tested version: 2.7.1. Other versions will most likely work as well though

- `PuTTY` on Windows.

Next, follow OS-agnostic installation instructions for a few of the tools:

### `rustc` & Cargo

Install rustup by following the instructions at [https://rustup.rs](https://rustup.rs).

If you already have rustup installed double check that you are on the stable
channel and your stable toolchain is up to date. `rustc -V` should return a date
newer than the one shown below:

``` console
$ rustc -V
rustc 1.45.2 (d3fb005a3 2020-07-31)
```

### `cargo-binutils`

``` console
$ rustup component add llvm-tools-preview

$ cargo install cargo-binutils --vers 0.3.1

$ cargo size -- -version
LLVM (http://llvm.org/):
  LLVM version 10.0.1-rust-1.45.2-stable
  Optimized build.
  Default target: x86_64-unknown-linux-gnu
  Host CPU: skylake
```

### `cargo-embed`

```console
$ cargo install cargo-embed --vers 0.9.0

$ cargo-embed --version
cargo-embed 0.9.0
```

### This repository

Since this book also contains some small Rust code bases used in various chapters
you will also have to download this books source code, you can do this in one of the following ways:

* Visit the [repository](https://github.com/rust-embedded/discovery/), click the green Code button and then the
   Download Zip one
* Clone it using git (if you know git you presumably already have it installed) from the same repository as linked in
   the zip approach

### OS specific instructions

Now follow the instructions specific to the OS you are using:

- [Linux](linux.md)
- [Windows](windows.md)
- [macOS](macos.md)
