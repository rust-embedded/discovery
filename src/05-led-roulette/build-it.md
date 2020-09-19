# Build it

The first step is to build our "binary" crate. Because the microcontroller has a different
architecture than your computer we'll have to cross compile. Cross compiling in Rust land is as simple
as passing an extra `--target` flag to `rustc`or Cargo. The complicated part is figuring out the
argument of that flag: the *name* of the target.

The microcontroller in the micro:bit has a Cortex-M0 processor in it. `rustc` knows how to cross compile
to the Cortex-M architecture and provides 4 different targets that cover the different processor
families within that architecture:

- `thumbv6m-none-eabi`, for the Cortex-M0 and Cortex-M1 processors
- `thumbv7m-none-eabi`, for the Cortex-M3 processor
- `thumbv7em-none-eabi`, for the Cortex-M4 and Cortex-M7 processors
- `thumbv7em-none-eabihf`, for the Cortex-M4**F** and Cortex-M7**F** processors

For the micro:bit, we'll use the `thumbv6m-none-eabi` target. Before cross compiling you have to
download pre-compiled version of the standard library (a reduced version of it actually) for your
target. That's done using `rustup`:

``` console
$ rustup target add thumbv6m-none-eabi
```

You only need to do the above step once; `rustup` will re-install a new standard library
(`rust-std` component) whenever you update your toolchain.

With the `rust-std` component in place you can now cross compile the program using Cargo:

``` console
$ # make sure you are in the `src/05-led-roulette` directory

$ cargo build --target thumbv6m-none-eabi
   Compiling semver-parser v0.7.0
   Compiling typenum v1.12.0
   Compiling proc-macro2 v1.0.19
   Compiling unicode-xid v0.2.1
   Compiling cortex-m v0.6.3
   (...)
   Compiling as-slice v0.1.3
   Compiling aligned v0.3.4
   Compiling cortex-m-rt-macros v0.1.8
   Compiling nrf-hal-common v0.11.1
    Finished dev [unoptimized + debuginfo] target(s) in 18.69s
```

> **NOTE** Be sure to compile this crate *without* optimizations.

> **NOTE** If you have looked into `.cargo/config` you will have noticed that the target
  is actually always set to "thumbv6m-none-eabi" so the --target flag to `cargo` can in
  fact be omitted here.

OK, now we have produced an executable. This executable won't blink any leds,
it's just a simplified version that we will build upon later in the chapter.
As a sanity check, let's verify that the produced executable is actually an ARM binary:

``` console
$ # equivalent to `readelf -h target/thumbv6m-none-eabi/debug/led-roulette`
 cargo readobj --target thumbv6m-none-eabi --bin led-roulette -- -file-headers
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0xC1
  Start of program headers:          52 (bytes into file)
  Start of section headers:          599484 (bytes into file)
  Flags:                             0x5000200
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         2
  Size of section headers:           40 (bytes)
  Number of section headers:         22
  Section header string table index: 20
```

Next, we'll flash the program into our microcontroller.
