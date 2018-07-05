# Build it

The first step is to build our "binary" crate. Because the microcontroller has a different
architecture than your laptop we'll have to cross compile. Cross compiling in Rust land is as simple
as passing an extra `--target` flag to `rustc`or Cargo. The complicated part is figuring out the
argument of that flag: the *name* of the target.

The microcontroller in the F3 has a Cortex-M4F processor in it. `rustc` knows how to cross compile
to the Cortex-M architecture and provides 4 different targets that cover the different processor
families within that architecture:

- `thumbv6m-none-eabi`, for the Cortex-M0 and Cortex-M1 processors
- `thumbv7m-none-eabi`, for the Cortex-M3 processor
- `thumbv7em-none-eabi`, for the Cortex-M4 and Cortex-M7 processors
- `thumbv7em-none-eabihf`, for the Cortex-M4**F** and Cortex-M7**F** processors

For the F3, we'll to use the `thumbv7em-none-eabihf` target. Before cross compiling you have to
download pre-compiled version of the standard library (a reduced version of it actually) for your
target. That's done using `rustup`:

``` console
$ rustup target add thumbv7em-none-eabihf
```

You only the above step once; `rustup` will re-install a new standard library (`rust-std` component)
whenever you update your toolchain.

With the `rust-std` component in place you can now cross compile the program using Cargo:

``` console
$ # make sure you are in the `src/05-led-roulette` directory

$ cargo build --target thumbv7em-none-eabihf
   Compiling semver-parser v0.7.0
   Compiling aligned v0.1.1
   Compiling libc v0.2.35
   Compiling bare-metal v0.1.1
   Compiling cast v0.2.2
   Compiling cortex-m v0.4.3
   (..)
   Compiling stm32f30x v0.6.0
   Compiling stm32f30x-hal v0.1.2
   Compiling aux5 v0.1.0 (file://$PWD/aux)
   Compiling led-roulette v0.1.0 (file://$PWD)
    Finished dev [unoptimized + debuginfo] target(s) in 35.84 secs
```

> **NOTE** Be sure to compile this crate *without* optimizations

OK, now we have produced an executable. As a sanity check, let's verify that the produced executable
is actually an ARM binary:

``` console
$ # *nix only
$ file target/thumbv7em-none-eabihf/debug/led-roulette
led-roulette: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, not stripped
$ #                                      ^^^  ^^^^^                   ^^^^^^^^^^^^^^^^^
```

Next, we'll flash the program into our microcontroller.
