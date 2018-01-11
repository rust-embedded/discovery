# Setting up a development environment

Dealing with microcontrollers involves several tools as we'll be dealing with an architecture
different than your laptop's and we'll have to run and debug programs on a "remote" device.

## Documentation

Tooling is not everything though. Without documentation is pretty much impossible to work with
microcontrollers.

We'll be referring to all these documents throughout this book:

*HEADS UP* All these links point to PDF files and some of them are hundreds of pages long and
several MBs in size.

- [STM32F3DISCOVERY User Manual][um]
- [STM32F303VC Datasheet][ds]
- [STM32F303VC Reference Manual][rm]
- [LSM303DLHC]
- [L3GD20]

[L3GD20]: http://www.st.com/resource/en/datasheet/l3gd20.pdf
[LSM303DLHC]: http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf
[ds]: http://www.st.com/resource/en/datasheet/stm32f303vc.pdf
[rm]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf
[um]: http://www.st.com/resource/en/user_manual/dm00063382.pdf

## Tools

We'll use all the tools listed below. Where a minimum version is not specified, any recent version
should work but we have listed the version we have tested.

- Cargo & `rustc` >= nightly-2018-01-01

- [Xargo] v0.3.10

- [`itmdump`] v0.2.1

- OpenOCD >=0.8. Tested versions: v0.9.0 and v0.10.0

- `arm-none-eabi-ld`. Tested version: 2.30

- `arm-none-eabi-gdb`. Version 7.12 or newer highly recommended. Tested versions: 7.10, 7.11,
  7.12 and 8.1

- `minicom` on Linux and macOS. Tested version: 2.7. Readers report that `picocom` also works but
  we'll use `minicom` in this text.

- `PuTTY` on Windows.

[Xargo]: https://crates.io/crates/xargo
[`itmdump`]: https://crates.io/crates/itm

If your laptop has Bluetooth functionality and you have the Bluetooth module, you can additionally
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

Then, install or switch to the nightly channel.

``` console
$ rustup default nightly
```

### Xargo

You can install Xargo in two different ways:

- By grabbing a [binary release] and placing it somewhere in your `$PATH`. `$HOME/.cargo/bin` is a
  good place to install it to. Do make sure that the binary release you "installed" actually works
  by executing the following command:

``` console
$ xargo -V
xargo 0.3.10
cargo 0.26.0-nightly (1d6dfea44 2018-01-26)
```

[binary release]: https://github.com/japaric/xargo/releases

- Or, by building it yourself with the following command:

``` console
$ cargo install xargo
$ xargo -V
xargo 0.3.10
cargo 0.26.0-nightly (1d6dfea44 2018-01-26)
```

You will additionally need to install the `rust-src` component (the source of the Rust compiler and
standard libraries) using `rustup` because Xargo depends on it:

``` console
$ rustup component add rust-src
```

### `itmdump`

``` console
$ cargo install itm --vers 0.2.1
```

### OS specific instructions

Now follow the instructions specific to the OS you are using:

- [Linux](03-setup/linux.html)
- [Windows](03-setup/windows.html)
- [macOS](03-setup/macos.html)
