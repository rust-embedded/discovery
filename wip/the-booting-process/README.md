# The booting process

We'll begin at the beginning: What's happens your microcontroller is powered up?
How does it know where the program starts? How much does it know how much RAM it
has available? etc.

But before that, we have to talk about memory.

## Memory and the address space

Cortex-M processors work with 32-bit addresses. Those addresses can go from
`0x0000_0000` all the way up to `0xFFFF_FFFF`. But not all these addresses can
be used in the same way (!). For Cortex-M processors, ARM dictates that this
address space must be semantically split in several regions:

```
+------------------------+ 0x0000_0000
|                        |
| Code                   |
|                        |
+------------------------+ 0x2000_0000
|                        |
| SRAM                   |
|                        |
+------------------------+ 0x4000_0000
|                        |
| Peripheral             |
|                        |
+------------------------+ 0x6000_0000
|                        |
| External RAM           |
|                        |
+------------------------+ 0xA000_0000
|                        |
| External device        |
|                        |
+------------------------+ 0xE000_0000
|                        |
| Private Peripheral Bus |
|                        |
+------------------------+ 0xE010_0000
|                        |
| Device                 |
|                        |
+------------------------+ 0xFFFF_FFFF
```

Each region has a different purpose and different characteristics, for example
the processor can never execute memory in the Peripheral region as if it
contained instructions.

---

### An aside: the hexadecimal format

You have noticed we have written the addresses in hexadecimal format and not in
decimal format. Why have we done that? For a few reasons:

- For consistency.

If I show these numbers: `0`, `4096` and `134217728`. Would you think these are
related in any way?

Probably not.

But if I instead show you: `0x0000_0000`, `0x0000_1000` and `0x0800_0000`. You
can easily tell that they are 32-bit numbers and you are more likely to think
that they are addresses. And if I mention the Cortex-M architecture, then you
may even guess that these are addresses in the same (Code) region.

- For ease of comparison. 

If I show you these addresses: `0x0800_0000` and `0x2000_1000`. And ask you: Do
these belong to the same memory region? You could just look at the two first
digits of each one, `0x08` and `0x20`, and answer: The first address belongs to
the Code region and the second one belongs to the SRAM region. Had I ask you the
same question using the decimal values, `134217728` and `536875008`, it would
have been much harder to answer.

---

We'll be mostly interested in the Code, the SRAM and the Peripheral regions. In
most cases, a whole region is not accessible by the processor because devices
have varying amounts of actual, physical memory.

To give an example: Our microcontroller, the STM32F303VCT6, has 256KiB of Flash
memory and that memory block resides in the Code region and spans from
`0x0800_0000` to `0x0804_0000`. This micro also has 40KiB of RAM. The RAM block,
as you'd expect, lives in the SRAM region and spans from `0x2000_0000` to
`0x2000_A000`.

As you may remember, this micro actually has 48 KiB of RAM. Where are the
remaining 8 KiB? Turns out that said 8 KiB memory block is actually located at
address `0x1000_0000` (!) which is kind of weird because that's part of the Code
region, not the SRAM region.

Finally, let's talk about the differences between Flash memory and RAM. Flash
memory is used for persistent storage and it's where we'll store our programs.
OTOH, RAM is "volatile": its contents are erased when power is removed and its
contents are random just after it's powered on. However, unlike Flash memory,
RAM is relatively easy to modify (write) and can accessed (read) randomly (hence
its name!); that's why RAM is used to store our programs' variables.

For the purposes of this workshop, we'll consider Flash memory as read only.
That is our programs won't modify its contents during their execution. Note that
Flash memory *is* writable otherwise we wouldn't be able to place our programs
there in the first place! And it is also possible to write a program that
modifies Flash memory though the procedure is rather involved.

## The boot process

OK. Now that you know about memory regions, here's what happens right after a
Cortex-M processor is powered on:

- The processor will read the (32-bit) value at address `0x0000_0000`, the
  initial Stack Pointer value, and initialize the stack pointer register (MSP)
  to this value.

- Then, the processor will read the (32-bit) value at address `0x0000_0004`, the
  reset "handler", and execute this handler.

That's it! Sounds simple enough. Let's elaborate on what these two steps entail.

Let's start with the second step. First, a *handler* is just ARM terminology for
*function pointer*. This means that what this step does is: load a function
pointer and execute it. Written as Rust code, that would be:

``` rust
// read the value at 0x0000_0004
let value = *(0x0000_0004 as *const u32);

// this is actually a function pointer
let reset_handler = value as fn();

// execute the function
reset_handler();
```

As for the first step, we must understand what "stack pointer" means. You see,
all (C-like?) programs use a data structure called the *call stack*, which is
kept in RAM, during their execution to keep track of the sub-routines
(functions) they execute.

In a nutshell: When the program calls a sub-routine it also pushes a new *frame*
into the call stack. This frame contains data local to the sub-routine (local
variables). When the program returns from the sub-routine, it will pop
(destroy) that frame thus freeing memory. (NOTE: I'm hand waving other details
like stacking registers and passing arguments through the stack)

How does the stack pointer fit into all this? Well, the stack pointer always
points into the current (top) frame thus is continuously updated throughout the
execution of the program.

So, what's the significance of the *initial* value of the stack pointer? Right
at the beginning of a program execution, the call stack is empty therefore this
initial value will dictate where the call stack will be allocated.

A picture to illustrate the concept of the call stack:

```
+-------------------+ 0x2000_0000 (start of the SRAM region)
|                   |
|                   |
        (...)
|                   |
|      (unused      |
|      memory)      |
|                   |
+-------------------+
|                   | 0x2000_9C80 (current value of the stack pointer)
|      frame 2      |
+-------------------+
|                   |
|                   |
|      frame 1      |
|                   |
+-------------------+
|                   |
|      frame 0      |
|                   |
+-------------------+ 0x2000_A000 (initial value of the stack pointer)
```

Note that:

- The frames may not have the same size
- The call stack grows downwards, towards smaller addresses. (This is an ancient
  convention.)

Unless you have a good reason not to, the end of the RAM block is what's usually
used as the initial Stack Pointer value. In principle, this allows the call
stack to use the whole RAM block. However, that won't likely be the case because
your program may also store `static` variables in RAM or make use of a "heap",
which also resides in RAM.

## Our first program

Armed with knowledge, let's build a minimal program for our microcontroller!

### The target

First thing we need to figure out is the *target* we have to use. We'll be using
the Rust compiler as a cross compiler therefore we need to tell it what
processor / architecture we want to produce code for and a target is used for
that.

There are four targets that cover the family of Cortex-M processors:

- `thumbv6m-none-eabi`, for Cortex-M0 and Cortex-M1 processors
- `thumbv7m-none-eabi`, for the Cortex-M3 processor
- `thumbv7em-none-eabi`, for the Cortex-M4 and Cortex-M7 processors
- `thumbv7em-none-eabihf`, for the Cortex-M4F and Cortex-M7F processors

Our microcontroller, the STM32F303VCT6, has a Cortex-M4F processor in it so the
right target is the `thumbv7em-none-eabihf` one.
 
### Take 1

We want to produce an executable and not a library so we start by creating a
"binary" Cargo project:

```
$ cargo new --bin app && cd $_
```

We want to build a `no_std` executable because the `std` crate doesn't support
our device and this *is* bare metal development! Our first attempt at a
`src/main.rs` will be this:

``` rust
#![no_std]

fn main() {}
```

```
$ xargo build --target thumbv7em-none-eabihf
   Compiling app v0.1.0 (file://$PWD)
error: language item required, but not found: `panic_fmt` 
```

OK, we need to provide the `panic_fmt` language (lang) item. This defines what
our program will do if a panic occurs.

### Take 2

``` rust
#![feature(lang_items)]
#![no_std]

fn main() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
```

```
$ xargo build --target thumbv7em-none-eabihf
   Compiling app v0.1.0 (file://$PWD)
error: requires `start` lang_item 
```

The `start` lang item is used to identify the entry point of our program which
must be a function with signature: `fn(isize, *const *const u8) -> isize`. But
that function is oriented towards command line applications, hence the
`*const *const u8` argument in it, which is actually the list of command line
arguments. Our program won't take any arguments; it's a bare metal program that
won't run under an OS and that won't be called via the command line!

So, instead of using `#[start]` (the `start` lang item), we'll use
`#![no_main]`. `#![no_main]` doesn't constrains us to an entry point with
command line-ish signature; it allows us to define a custom entry point.

### Take 3

``` rust
#![feature(lang_items)]
#![no_main]
#![no_std]

fn main() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
```


```
$ xargo build --target thumbv7em-none-eabihf
   Compiling app v0.1.0 (file://$PWD)
warning: function is never used: `main`, #[warn(dead_code)] on by default
 --> src/main.rs:5:1
  |
5 | fn main() {}
  | ^^^^^^^^^^^^

error: linking with `arm-none-eabi-gcc` failed: exit code: 1
  |
  = note: "arm-none-eabi-gcc" \
          "-L" \
          "$sysroot/lib/rustlib/thumbv7em-none-eabihf/lib" \
          "$PWD/target/thumbv7em-none-eabihf/debug/app.0.o" \
          "-o" \
          "$PWD/target/thumbv7em-none-eabihf/debug/app" \
          "-Wl,--gc-sections" \
          "-nodefaultlibs" \
          "-L" \
          "$PWD/target/thumbv7em-none-eabihf/debug/deps" \
          "-L" \
          "$sysroot/lib/rustlib/thumbv7em-none-eabihf/lib" \
          "-Wl,-Bstatic" \
          "-Wl,-Bdynamic" \
          "$sysroot/lib/rustlib/thumbv7em-none-eabihf/lib/libcore.rlib"
  = note: ld: cannot find crt0.o: No such file or directory
collect2: error: ld returned 1 exit status
```

> **NOTE** If you have `newlib` installed, you'll run into a different linker
> error: A bunch of undefined errors: `memset`, `exit`, etc.

What's this `crt0.o`? That's a startup object; `gcc` implicitly passes that to
the linker (`ld`). We don't want `gcc` to inject this C stuff into our
executable so we'll pass `-nostartfiles` to the linker to prevent that.

### Take 4

The simplest way is to use `xargo rustc` with the `-C link-arg=-nostartfiles`
flag:

```
$ xargo rustc --target thumbv7em-none-eabihf -- \
  -C link-arg=-nostartfiles
   Compiling app v0.1.0 (file://$PWD)
warning: function is never used: `main`, #[warn(dead_code)] on by default
 --> src/main.rs:5:1
  |
5 | fn main() {}
  | ^^^^^^^^^^^^
```

Success! Or not? That `dead_code` warning looks suspicious.

In bare metal development, you always want to inspect your binary because if you
flash an *invalid* binary into your device you could brick it.

Let's start with a sanity check using `file` (*nix only):

```
$ file target/thumbv7em-none-eabihf/debug/app
app: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, not stripped
         ~~~~~~                 ~~~                          ~~~~~~~~~~~~~~~~~
```

We got a statically linked ARM binary. That's definitively a good start.

In the previous subsection, we talked about memory regions and about certain
memory regions being important for the boot process. `objdump` is the tool you
want to use to explore the memory layout of your program:

```
# -C: demangle (AKA prettify the output)
# -d: disassemble (AKA show the "instructions")
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/debug/app

target/thumbv7em-none-eabihf/debug/app:     file format elf32-littlearmp
```

And ... our binary appears to be empty!

What happened here is that the linker dropped our `main` function (symbol)
because it will never get called hence it's dead code. The thing is that, by
default, the entry point of a program is the `_start` symbol (function) but our
crate doesn't define it.

---

### An aside: how `rustc` produces executables

Like other compilers, `rustc` produces executables in two steps: compilation and
linking. Compilation is the process of translating a source file (`.rs`) into an
intermediate object file (`.o`). And linking is the process of merging one or
more `.o` files into a single object file: the executable.

In Rust, each crate maps to a single object file. In fact, a `.rlib` is an
object file that has been archived (using `ar`) alongside some extra metadata.

---

You can check the symbols that our crate exposes *before* the linker drops them
by emitting an object file with `--emit=obj`:

```
$ xargo rustc --target thumbv7em-none-eabihf -- \
  -C link-arg=-nostartfiles --emit=obj
```

You'll find an extra `.o` file right next to the executable. Running that
through `objdump`:

```
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/debug/app.o

target/thumbv7em-none-eabihf/debug/app.o:     file format elf32-littlearm


Disassembly of section .text._ZN3app4main17hdf94c05b43b60536E:

00000000 <app::main::hdf94c05b43b60536>:
   0:   b081            sub     sp, #4
   2:   e7ff            b.n     4 <app::main::hdf94c05b43b60536+0x4>
   4:   b001            add     sp, #4
   6:   4770            bx      lr

Disassembly of section .text.rust_begin_unwind:

00000000 <rust_begin_unwind>:
   0:   b081            sub     sp, #4
   2:   e7ff            b.n     4 <rust_begin_unwind+0x4>
   4:   b001            add     sp, #4
   6:   4770            bx      lr
```

This is a great opportunity to learn to read the output of `objdump`. At its
core, `objdump` outputs a list of symbols. Each symbol has a start address, a
name and a list of values. If the symbol is a function/routine then the list of
values is a list of instructions. The syntax of each symbol is the following:

```
[START_ADDRESS] <[SYMBOL_NAME]>:
[OFFSET]:   [VALUE]        [INSTRUCTION]
[OFFSET]:   [VALUE]        [INSTRUCTION]
[OFFSET]:   [VALUE]        [INSTRUCTION]
```

Looking at the `objdump` output, the main take away is that this object file
exposes two symbols:

- `app::main::hdf94c05b43b60536`. This is our `main` function.
- `rust_begin_unwind`. This is the `panic_fmt` lang item.

As I mentioned before, the issue is that the linker expects the entry point
(symbol) to be named `_start` but our crate doesn't provide that symbol so the
linker just drop every other symbol.

### Take 5

Let's expose a `_start` symbol:

```
#![feature(lang_items)]
#![no_main]
#![no_std]

#[export_name = "_start"]
fn main() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
```

```
$ xargo rustc --target thumbv7em-none-eabihf -- -C link-arg=-nostartfiles --emit=obj
   Compiling app v0.1.0 (file://$PWD)
warning: function is never used: `main`, #[warn(dead_code)] on by default
 --> src/main.rs:6:1
  |
6 | fn main() {}
  | ^^^^^^^^^^^^ 
```

And ...

```
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/debug/app

target/thumbv7em-none-eabihf/debug/app:     file format elf32-littlearm 
```

our executable is still empty. What went wrong this time?

```
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/debug/app.o

target/thumbv7em-none-eabihf/debug/app.o:     file format elf32-littlearm


Disassembly of section .text._start:

00000000 <_start>:
   0:   b081            sub     sp, #4
   2:   e7ff            b.n     4 <_start+0x4>
   4:   b001            add     sp, #4
   6:   4770            bx      lr

Disassembly of section .text.rust_begin_unwind:

00000000 <rust_begin_unwind>:
   0:   b081            sub     sp, #4
   2:   e7ff            b.n     4 <rust_begin_unwind+0x4>
   4:   b001            add     sp, #4
   6:   4770            bx      lr 
```

Our crate *does* export a `_start` symbol. But! This time, the issue is that the
`_start` symbol exposed by our crate is local rather than global. You can check
this using a new tool: `nm`.

```
$ arm-none-eabi-nm target/thumbv7em-none-eabihf/debug/app.o
00000000 N
00000034 N
00000042 N
00000058 N
0000005c N
0000006a N
0000006f N
00000082 N
         U __aeabi_unwind_cpp_pr0
00000000 T rust_begin_unwind
00000000 N __rustc_debug_gdb_scripts_section__
00000000 t _start 
```

The `t` right new to `_start` indicates that is a *local* symbol rather than a
*global* one. The entry point has to be a global symbol.

### Take 6

```
#![feature(lang_items)]
#![no_main]
#![no_std]

#[export_name = "_start"]
pub fn main() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
```

```
$ xargo rustc --target thumbv7em-none-eabihf -- -C link-arg=-nostartfiles
   Compiling app v0.1.0 (file://$PWD)
error: linking with `arm-none-eabi-gcc` failed: exit code: 1
  |
  = note: "arm-none-eabi-gcc" \
          "-L" \
          "$sysroot/lib/rustlib/thumbv7em-none-eabihf/lib" \
          "$PWD/target/thumbv7em-none-eabihf/debug/app.0.o" \
          "-o" \
          "$PWD/target/thumbv7em-none-eabihf/debug/app" \
          "-Wl,--gc-sections" \
          "-nodefaultlibs" \
          "-L" \
          "$PWD/target/thumbv7em-none-eabihf/debug/deps" \
          "-L" \
          "$sysroot/lib/rustlib/thumbv7em-none-eabihf/lib" \
          "-Wl,-Bstatic" \
          "-Wl,-Bdynamic" \
          "$sysroot/lib/rustlib/thumbv7em-none-eabihf/lib/libcore.rlib" \
          "-nostartfiles"
  = note: $PWD/target/thumbv7em-none-eabihf/debug/app.0.o:(.ARM.exidx.text._start+0x0): undefined reference to `__aeabi_unwind_cpp_pr0'
collect2: error: ld returned 1 exit status
```

An error! That's what I call progress.

"undefined reference to `__aeabi_unwind_cpp_pr0'". We are definitively missing a
symbol. The symbol seems related to unwinding but we are using panic=abort (the
thumb targets default to that) so we'll just stub the symbol.

### Take 7

```
#![feature(lang_items)]
#![no_main]
#![no_std]

#[export_name = "_start"]
pub fn main() {}

#[export_name = "__aeabi_unwind_cpp_pr0"]
fn __aeabi_unwind_cpp_pr0() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
```

```
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/debug/app

target/thumbv7em-none-eabihf/debug/app:     file format elf32-littlearm


Disassembly of section .text:

00008000 <_start>:
    8000:       b081            sub     sp, #4
    8002:       e7ff            b.n     8004 <_start+0x4>
    8004:       b001            add     sp, #4
    8006:       4770            bx      lr

00008008 <__aeabi_unwind_cpp_pr0>:
    8008:       b081            sub     sp, #4
    800a:       e7ff            b.n     800c <__aeabi_unwind_cpp_pr0+0x4>
    800c:       b001            add     sp, #4
    800e:       4770            bx      lr
```

Hurray! This time our executable is not empty! However, if we flash this program
into our microcontroller our device won't boot!

We saw in the previous subsections that the boot process requires the start of
the Code region to be initialized in a certain manner but our executable doesn't
even mention that region (`0x0000_0000`).

We need more control over the memory layout of our program. Enter: linker
scripts.

### Take 8

Linker scripts instruct the linker where to place symbols. We won't cover them
in depth here but the following linker script, which hopefully is not that hard
to read, will place the symbols (values), required for the boot process, in the
right place:

```
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    /* Initial Stack Pointer value */
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    /* Reset handler */
    LONG(_reset + 1);

    /* Entry point: the reset handler */
    _reset = .;
    KEEP(*(.text._start));

    *(.text.*);
  } > FLASH
}
```

```
$ xargo rustc --target thumbv7em-none-eabihf -- \
  -C link-arg=-Tlayout.ld -C link-arg=-nostartfiles
```

```
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/debug/app

target/thumbv7em-none-eabihf/debug/app:     file format elf32-littlearm


Disassembly of section .text:

08000000 <_start-0x8>:
 8000000:       2000a000        .word   0x2000a000
 8000004:       08000009        .word   0x08000009

08000008 <_start>:
 8000008:       b081            sub     sp, #4
 800000a:       e7ff            b.n     800000c <_start+0x4>
 800000c:       b001            add     sp, #4
 800000e:       4770            bx      lr

08000010 <__aeabi_unwind_cpp_pr0>:
 8000010:       b081            sub     sp, #4
 8000012:       e7ff            b.n     8000014 <__aeabi_unwind_cpp_pr0+0x4>
 8000014:       b001            add     sp, #4
 8000016:       4770            bx      lr
```

Now, you may remember that the boot process involves the memory around the
address `0x0000_0000` but our executable doesn't touch that chunk of memory so
you, with just reason, will think that this still is an invalid binary. But
there's a catch here: STM32 microcontrollers alias the memory region block at
`0x0800_0000` at the `0x0000_0000` address. This means that reading the address
`0x0000_0000` is the same as reading the address `0x0800_0000`.

With that knowledge, you can determine what will happen during the boot process:

- `0x0000_0000` -> `0x2000_a000` = `0x2000_0000` + 40 KiB. This is the end of
  the RAM region. The initial Stack Pointer value is the highest available
  RAM address.
- `0x0000_0000` -> `0x0800_0009`. 

> **TODO** rest of the boot process. Mention the vector table

### Take 9

One last change! The reset handler, `main`, is the first function that the
processor calls and it will be rooted at the start of the call stack. Would it
make sense to return from `main`? No! There's nothing to return to! Returning
from the reset handler is undefined. Can we statically prevent that? Yes! `!`,
the bottom type, to the rescue:

```
#![feature(lang_items)]
#![no_main]
#![no_std]

#[export_name = "_start"]
pub fn main() -> ! {
    loop {}
}

#[export_name = "__aeabi_unwind_cpp_pr0"]
fn __aeabi_unwind_cpp_pr0() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
```

By changing the signature of `main` to `fn() -> !`, that is by making it a
*diverging* function, we have made it statically impossible to return from it.

There are two ways to "end" a diverging function: with an infinite `loop` or
with a call to another diverging function. Here, we use an infinite loop.

Binary size is another important metric. The `size` tool can be used to look
into that:

```
$ arm-none-eabi-size target/thumbv7em-none-eabihf/debug/app                                                                         <<<
   text    data     bss     dec     hex filename
     40       0       0      40      28 target/thumbv7em-none-eabihf/debug/app
```

The output shows three different sections: `text`, `data` and `bss`. `dec` is
just the sum of these three sections and `hex` is the same as `dec` but in
hexadecimal format.

You have already seen the `text` section. It came up in the linker script.

> **TODO** explain all the sections in the output of `size`
