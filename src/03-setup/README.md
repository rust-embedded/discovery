# Setting up a development environment

Dealing with microcontrollers involves several tools as we'll be dealing with an
architecture different than your laptop's and we'll have to run and debug
programs on a "remote" device.

## Documentation

Tooling is not everything though. Without documentation is pretty much
impossible to work with microcontrollers (unless you are very good at reverse
engineering and even then it would be a *lot* more work).

We'll be referring to all these documents throughout this book:

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

We'll use all the tools listed below. Where a minimum version is not specified,
any recent version should work but we have listed the version we have tested.

- Cargo & `rustc`  >= nightly-2016-10-05

- [Xargo] >= 0.1.13. But 0.2.x is highly recommended.

- [`itmdump`] >= 0.1.1

- OpenOCD >=0.8. Tested version: 0.9.0

- `arm-none-eabi-gcc`. Tested versions: 4.8, 5.2 and 6.2

- `arm-none-eabi-gdb`. Version 7.12 or newer highly recommended. Tested
  versions: 7.10, 7.11 and 7.12

- `minicom` on Linux and macOS. Tested version: 2.7. Readers report that
  `picocom` also works but we'll use `minicom` in this text.

- `PuTTY` on Windows.

[Xargo]: https://crates.io/crates/xargo
[`itmdump`]: https://crates.io/crates/itm

If your laptop has Bluetooth functionality and you have the Bluetooth module,
you can additionally install these tools to play with the Bluetooth module we'll
be providing. All these are optional:

- Linux, only if you don't have a Bluetooth manager application like Blueman.
  - `bluez`
  - `hcitool`
  - `rfcomm`
  - `rfkill`

macOS / OSX / Windows users only need the default bluetooth manager that ships
with their OS.

Next, follow OS-agnostic installation instructions for a few of the tools:

### `rustc` & Cargo

Install rustup by following the instructions at https://rustup.rs.

Then, install or switch to the nightly channel.

```
$ rustup default nightly
```

### Xargo

You can install Xargo in two different ways:

- By grabbing a [binary release] and placing it somewhere in your `$PATH`.
  `$HOME/.cargo/bin` is a good place to install it to. Do make sure that the
  binary release you "installed" actually works by executing the following
  command:

```
$ xargo -V
xargo 0.2.0 (bd8ebc4 2016-10-16)
cargo 0.13.0-nightly (a8baa5b 2016-10-15)
```

[binary release]: https://github.com/japaric/xargo/releases

- Or, by building it yourself with the following command:

```
$ cargo install xargo
$ xargo -V
```

You will additionally need to install the `rust-src` component (the source of
the Rust compiler and standard libraries) using `rustup` because Xargo (v0.2.0+)
depends on it:

```
$ rustup component add rust-src
```

### `itmdump`

```
$ cargo install itm
```

### OS specific instructions

- [Linux](03-setup/linux.html)
- [Windows](03-setup/windows.html)
- [macOS](03-setup/macos.html)
