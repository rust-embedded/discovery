# Build it

The first step is to build our "binary" crate. Because the microcontroller has
a different architecture than your laptop we'll have to cross compile. Cross
compiling in Rust land is as simple as passing an extra `--target` flag to
`rustc`or Cargo. The complicated part is figuring out the argument of that flag:
the *name* of the target.

The microcontroller in the F3 has a Cortex-M4F processor in it. `rustc` knows
how to cross compile to the Cortex-M architecture and provides 4 different
targets that cover the different processor families within that architecture:

- `thumbv6m-none-eabi`, for the Cortex-M0 and Cortex-M1 processors
- `thumbv7m-none-eabi`, for the Cortex-M3 processor
- `thumbv7em-none-eabi`, for the Cortex-M4 and Cortex-M7 processors
- `thumbv7em-none-eabihf`, for the Cortex-M4**F** and Cortex-M7**F** processors

For the F3, we'll to use the `thumbv7em-none-eabihf` target.

Now, `rustup` doesn't provide a binary release of the `core` crate for this
target, so we'll use Xargo instead of Cargo. Xargo will take care of compiling
the `core` crate for us:

```
$ xargo build --target thumbv7em-none-eabihf
   Compiling core v0.0.0 (file://$SYSROOT/lib/rustlib/src/rust/src/libcore)
   Compiling alloc v0.0.0 (file://$SYSROOT/lib/rustlib/src/rust/src/liballoc)
   Compiling rustc_unicode v0.0.0 (file://$SYSROOT/lib/rustlib/src/rust/src/librustc_unicode)
   Compiling collections v0.0.0 (file://$SYSROOT/lib/rustlib/src/rust/src/libcollections)
   Compiling rand v0.0.0 (file://$SYSROOT/lib/rustlib/src/rust/src/librand)
   Compiling f3 v0.3.0
   Compiling rlibc v1.0.0
   Compiling r0 v0.1.0
   Compiling volatile-register v0.1.2
   Compiling cortex-m v0.1.4
   Compiling compiler-builtins-snapshot v0.0.20161008+c56faf22abb39724008148d58f12bcd43b6d236b
   Compiling pg v0.1.0 (file://$SYSROOT/04-led-roulette/pg)
   Compiling led-roulette v0.1.0 (file://$SYSROOT/04-led-roulette)
```

> **NOTE** Be sure to compile this crate *without* optimizations

Also, let me note that Xargo exposes the exact same UI as Cargo so you can use
any subcommand (even custom ones) that you would normally use with Cargo.

OK, now we have produced an executable. As a sanity check, let's verify that
the produced executable is actually an ARM binary:

```
# *nix only
$ file target/thumbv7em-none-eabihf/debug/led-roulette
led-roulette: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, not stripped
                                         ~~~  ~~~~~                   ~~~~~~~~~~~~~~~~~
```

Another way to do that is to use `readelf` because the executable produced by
`rustc` is actually an ELF (Executable and Linkable Format) file.

```
$ arm-none-eabi-readelf -h target/thumbv7em-none-eabihf/debug/led-roulette
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM <--
  Version:                           0x1
  Entry point address:               0x8000195 <--
  Start of program headers:          52 (bytes into file)
  Start of section headers:          555816 (bytes into file)
  Flags:                             0x5000400, Version5 EABI, hard-float ABI <--
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         2
  Size of section headers:           40 (bytes)
  Number of section headers:         17
  Section header string table index: 14
```

Next, we'll "flash" the program into our microcontroller.
