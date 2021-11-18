# Build it

The first step is to build our "binary" crate. Because the microcontroller has a different
architecture than your computer we'll have to cross compile. Cross compiling in Rust land is as simple
as passing an extra `--target` flag to `rustc`or Cargo. The complicated part is figuring out the
argument of that flag: the *name* of the target.

The microcontroller in the F3 has a Cortex-M4F processor in it. `rustc` knows how to cross compile
to the Cortex-M architecture and provides 4 different targets that cover the different processor
families within that architecture:

- `thumbv6m-none-eabi`, for the Cortex-M0 and Cortex-M1 processors
- `thumbv7m-none-eabi`, for the Cortex-M3 processor
- `thumbv7em-none-eabi`, for the Cortex-M4 and Cortex-M7 processors
- `thumbv7em-none-eabihf`, for the Cortex-M4**F** and Cortex-M7**F** processors

For the F3, we'll use the `thumbv7em-none-eabihf` target. Before cross compiling you have to
download a pre-compiled version of the standard library (a reduced version of it actually) for your
target. That's done using `rustup`:

``` console
rustup target add thumbv7em-none-eabihf
```

You only need to do the above step once; `rustup` will re-install a new standard library
(`rust-std` component) whenever you update your toolchain.

With the `rust-std` component in place you can now cross compile the program using Cargo.

> **NOTE** Make sure you are in the `src/05-led-roulette` directory
> and run `cargo build` command below to create the executable:
``` console
cargo build --target thumbv7em-none-eabihf
```
On your console you should see something like:
``` console
$ cargo build --target thumbv7em-none-eabihf
   Compiling typenum v1.12.0
   Compiling semver-parser v0.7.0
   Compiling version_check v0.9.2
   Compiling nb v1.0.0
   Compiling void v1.0.2
   Compiling autocfg v1.0.1
   Compiling cortex-m v0.7.1
   Compiling proc-macro2 v1.0.24
   Compiling vcell v0.1.3
   Compiling unicode-xid v0.2.1
   Compiling stable_deref_trait v1.2.0
   Compiling syn v1.0.60
   Compiling bitfield v0.13.2
   Compiling cortex-m v0.6.7
   Compiling cortex-m-rt v0.6.13
   Compiling r0 v0.2.2
   Compiling stm32-usbd v0.5.1
   Compiling stm32f3 v0.12.1
   Compiling usb-device v0.2.7
   Compiling cfg-if v1.0.0
   Compiling paste v1.0.4
   Compiling stm32f3-discovery v0.6.0
   Compiling embedded-dma v0.1.2
   Compiling volatile-register v0.2.0
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling semver v0.9.0
   Compiling generic-array v0.14.4
   Compiling switch-hal v0.3.2
   Compiling num-traits v0.2.14
   Compiling num-integer v0.1.44
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling cast v0.2.3
   Compiling quote v1.0.9
   Compiling generic-array v0.13.2
   Compiling generic-array v0.12.3
   Compiling generic-array v0.11.1
   Compiling panic-itm v0.4.2
   Compiling lsm303dlhc v0.2.0
   Compiling as-slice v0.1.4
   Compiling micromath v1.1.0
   Compiling accelerometer v0.12.0
   Compiling chrono v0.4.19
   Compiling aligned v0.3.4
   Compiling rtcc v0.2.0
   Compiling cortex-m-rt-macros v0.1.8
   Compiling stm32f3xx-hal v0.6.1
   Compiling aux5 v0.2.0 (~/embedded-discovery/src/05-led-roulette/auxiliary)
   Compiling led-roulette v0.2.0 (~/embedded-discovery/src/05-led-roulette)
    Finished dev [unoptimized + debuginfo] target(s) in 17.91s
```

> **NOTE** Be sure to compile this crate *without* optimizations. The provided Cargo.toml file and build command above will ensure optimizations are off.

OK, now we have produced an executable. This executable won't blink any LEDs, it's just a simplified version that we will build upon later in the chapter. As a sanity check, let's verify that the produced executable is actually an ARM binary:

``` console
cargo readobj --target thumbv7em-none-eabihf --bin led-roulette -- --file-header
```
The `cargo readobj ..` above is equivalent to
`readelf -h target/thumbv7em-none-eabihf/debug/led-roulette`
and should produce something similar to:
``` console
$ cargo readobj --target thumbv7em-none-eabihf --bin led-roulette -- --file-header
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
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
  Entry point address:               0x8000195
  Start of program headers:          52 (bytes into file)
  Start of section headers:          818328 (bytes into file)
  Flags:                             0x5000400
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         4
  Size of section headers:           40 (bytes)
  Number of section headers:         22
  Section header string table index: 20
  ```

Next, we'll flash the program into our microcontroller.
