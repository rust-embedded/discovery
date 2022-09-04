# Build it

The first step is to build our "binary" crate. Because the microcontroller has a different
architecture than your computer we'll have to cross compile. Cross compiling in Rust land is as simple
as passing an extra `--target` flag to `rustc`or Cargo. The complicated part is figuring out the
argument of that flag: the *name* of the target.

As we already know the microcontroller on the micro:bit v2 has a Cortex-M4F processor in it, the one on v1 a Cortex-M0.
`rustc` knows how to cross-compile to the Cortex-M architecture and provides several different targets that cover the different processors
families within that architecture:

- `thumbv6m-none-eabi`, for the Cortex-M0 and Cortex-M1 processors
- `thumbv7m-none-eabi`, for the Cortex-M3 processor
- `thumbv7em-none-eabi`, for the Cortex-M4 and Cortex-M7 processors
- `thumbv7em-none-eabihf`, for the Cortex-M4**F** and Cortex-M7**F** processors
- `thumbv8m.main-none-eabi`, for the Cortex-M33 and Cortex-M35P processors
- `thumbv8m.main-none-eabihf`, for the Cortex-M33**F** and Cortex-M35P**F** processors

For the micro:bit v2, we'll use the `thumbv7em-none-eabihf` target, for v1 the `thumbv6m-none-eabi` one.
Before cross-compiling you have to download a pre-compiled version of the standard library
(a reduced version of it, actually) for your target. That's done using `rustup`:

``` console
# For micro:bit v2
$ rustup target add thumbv7em-none-eabihf
# For micro:bit v1
$ rustup target add thumbv6m-none-eabi
```

You only need to do the above step once; `rustup` will re-install a new standard library
(`rust-std` component) whenever you update your toolchain. Therefore you can skip this step, if you have already added the necessary target
while [verifying your setup].

[verifying your setup]: ../03-setup/verify.html#verifying-cargo-embed


With the `rust-std` component in place you can now cross compile the program using Cargo:

``` console
# make sure you are in the `src/05-led-roulette` directory

# For micro:bit v2
$ cargo build --features v2 --target thumbv7em-none-eabihf
   Compiling semver-parser v0.7.0
   Compiling typenum v1.12.0
   Compiling cortex-m v0.6.3
   (...)
   Compiling microbit-v2 v0.10.1
    Finished dev [unoptimized + debuginfo] target(s) in 33.67s

# For micro:bit v1
$ cargo build --features v1 --target thumbv6m-none-eabi
   Compiling fixed v1.2.0
   Compiling syn v1.0.39
   Compiling cortex-m v0.6.3
   (...)
   Compiling microbit v0.10.1
	Finished dev [unoptimized + debuginfo] target(s) in 22.73s
```

> **NOTE** Be sure to compile this crate *without* optimizations. The provided Cargo.toml
> file and build command above will ensure optimizations are off.

OK, now we have produced an executable. This executable won't blink any LEDs,
it's just a simplified version that we will build upon later in the chapter.
As a sanity check, let's verify that the produced executable is actually an ARM binary:

``` console
# For micro:bit v2
# equivalent to `readelf -h target/thumbv7em-none-eabihf/debug/led-roulette`
$ cargo readobj --features v2 --target thumbv7em-none-eabihf --bin led-roulette -- --file-headers
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
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
  Entry point address:               0x117
  Start of program headers:          52 (bytes into file)
  Start of section headers:          793112 (bytes into file)
  Flags:                             0x5000400
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         4
  Size of section headers:           40 (bytes)
  Number of section headers:         21
  Section header string table index: 19

# For micro:bit v1
# equivalent to `readelf -h target/thumbv6m-none-eabi/debug/led-roulette`
$ cargo readobj --features v1 --target thumbv6m-none-eabi --bin led-roulette -- --file-headers
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
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
  Start of section headers:          693196 (bytes into file)
  Flags:                             0x5000200
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         4
  Size of section headers:           40 (bytes)
  Number of section headers:         22
  Section header string table index: 20
```

Next, we'll flash the program into our microcontroller.
