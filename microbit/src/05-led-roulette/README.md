# LED roulette

Alright, let's start by building the following application:

<p align="center">
<video src="../assets/roulette_fast.mp4" loop autoplay>
</p>

I'm going to give you a high level API to implement this app but don't worry we'll do low level
stuff later on. The main goal of this chapter is to get familiar with the *flashing* and debugging
process.

The starter code is in the `src` directory of the book repository. Inside that directory there are more
directories named after each chapter of this book. Most of those directories are starter Cargo
projects.

Now, jump into the `src/05-led-roulette` directory. Check the `src/main.rs` file:

``` rust
{{#include src/main.rs}}
```

Microcontroller programs are different from standard programs in two aspects: `#![no_std]` and
`#![no_main]`.

The `no_std` attribute says that this program won't use the `std` crate, which assumes an underlying
OS; the program will instead use the `core` crate, a subset of `std` that can run on bare metal
systems (i.e., systems without OS abstractions like files and sockets).

The `no_main` attribute says that this program won't use the standard `main` interface, which is
tailored for command line applications that receive arguments. Instead of the standard `main` we'll
use the `entry` attribute from the [`cortex-m-rt`] crate to define a custom entry point. In this
program we have named the entry point "main", but any other name could have been used. The entry
point function must have signature `fn() -> !`; this type indicates that the function can't return
-- this means that the program never terminates.

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

If you are a careful observer, you'll also notice there is a `.cargo` directory in the Cargo project
as well. This directory contains a Cargo configuration file (`.cargo/config`) that tweaks the
linking process to tailor the memory layout of the program to the requirements of the target device.
This modified linking process is a requirement of the `cortex-m-rt` crate.

Furthermore, there is also an `Embed.toml` file

```toml
{{#include Embed.toml}}
```

This file tells `cargo-embed` that:

* we are working with either a nrf52833 or nrf51822, you will again have to remove the comments from the
  chip you are using, just like you did in chapter 3.
* we want to halt the chip after we flashed it so our program does not instantly jump to the loop
* we want to disable RTT, RTT being a protocol that allows the chip to send text to a debugger.
  You have in fact already seen RTT in action, it was the protocol that sent "Hello World" in chapter 3.
* we want to enable GDB, this will be required for the debugging procedure

Alright, let's start by building this program.
