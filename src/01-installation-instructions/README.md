# Installation I

## CHANGELOG

Updates since your last visit:

- All: Install [Xargo 0.2.0] or newer. The installation process is much simpler
  on this new version. Do note that with Xargo 0.2.0, it's mandatory to install
  the `rust-src` component using `rustup`.
- Windows: Be sure to install [the USB driver for the ST-LINK].

[Xargo 0.2.0]: 01-installation-instructions/README.html#Xargo
[the USB driver for the ST-LINK]: 01-installation-instructions/windows.html#ST-LINK%20USB%20driver

---

Let's install a bunch of tools that we'll use during the workshop!

> Please complete these instructions **before** the day of the workshop!

If you run into any problem while following these instructions, feel free to
open [an issue], [shoot me an e-mail] or ping me (`japaric`) on Mozilla's IRC
network (I'm on pretty much every Rust related channel but if you to have pick
one, I'd recommend the #rust-embedded channel).

The Rust Belt Rust organizers have also set up a slack "team" for the attendees
and speakers: [rust-belt-rust.slack.com]. Within that team, there's an
[#embedded-workshop] channel for our use during the workshop. You can also ask
me questions about the installation process over there!

[rust-belt-rust.slack.com]: https://rust-belt-rust.slack.com
[#embedded-workshop]: https://rust-belt-rust.slack.com/messages/embedded-workshop/

[an issue]: https://github.com/japaric/rbr2016/issues
[shoot me an e-mail]: https://github.com/japaric

I also recommend subscribing to [this repository] so you get notified of any
change in the installation process.

[this repository]: https://github.com/japaric/rbr2016

## Documentation

Download a copy of the [documentation](../#Documentation) and keep them
somewhere handy. We'll refer to them several times during the workshop!

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
- `minicom` on Linux and macOS. Tested version: 2.7
- `PuTTY` on Windows.

[Xargo]: https://crates.io/crates/xargo
[`itmdump`]: https://crates.io/crates/itm

If the laptop you'll be using for the workshop has Bluetooth, you can
additionally install these tools to play with the Bluetooth module we'll be
providing. All these are optional:

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

Then, install or switch to the nightly channel (2016-10-05 or newer)

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
the Rust compiler and standard libraries) using `rustup` because Xargo (v0.2.x)
depends on it:

```
$ rustup component add rust-src
```

### `itmdump`

```
$ cargo install itm
```

> **NOTE** Windows users will need to install a version 0.1.1 or newer because
> version 0.1.0 doesn't support Windows.

### OS specific instruction

- [Linux](01-installation-instructions/linux.html)
- [Windows](01-installation-instructions/windows.html)
- [macOS](01-installation-instructions/macos.html)
