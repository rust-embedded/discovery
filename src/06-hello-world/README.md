# Hello, world!

> **HEADS UP** Several readers have reported that the "solder bridge" SB10 (see
> back of the board) on the STM32F3DISCOVERY, which is required to use the ITM
> and the `iprint!` macros shown below, is **not** soldered even though
> the [User Manual][] (page 21) says that it **should be**. TL;DR You have two
> options to fix this: Either **solder** the solder bridge SB10 or connect a
> wire between SWO and PB3 as shown in the picture below.

[User Manual]: http://www.st.com/resource/en/user_manual/dm00063382.pdf

<p align="center">
<img height=640 title="Manual SWD connection" src="assets/f3-swd.png">
</p>

---

(Just a little more of helpful "magic" before we start doing low level stuff.)

Blinking an LED is like the "Hello, world" of the embedded world.

But in this section, we'll run a proper "Hello, world" program that prints stuff
to the console.

Go to the `06-hello-world` directory. There's some starter code in it:

``` rust
#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[macro_use]
extern crate pg;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    iprintln!("Hello, world!");

    loop {}
}
```

The `iprintln` macro will format messages and output them to the
microcontroller's *ITM*. ITM stands for Instrumentation Trace Macrocell and it's
a communication protocol on top of SWD (Serial Wire Debug) which can be used to
send messages from the microcontroller to the debugging host. This communication
is only "one way" as the debugging host can't send data to the microcontroller.

OpenOCD, which is managing the debug session, can receive data sent through this
"ITM channel" and redirect it to a file.

The ITM protocol works with "frames" (you can think of them as ethernet
packets). Each frame has a header and a variable length payload. OpenOCD will
receive these frames and write them directly to a file without parsing them. So,
if the microntroller sends the string "Hello, world!" using the `iprintln`
macro, OpenOCD's output file won't exactly contain that string.

To retrieve the original string, OpenOCD's output file will have to be parsed.
We'll use the `itmdump` program to perform the parsing "on the fly".

You should have already installed the `itmdump` program during the [installation
chapter].

[installation chapter]: 03-setup/README.html#itmdump

In a new terminal, run this command inside the `/tmp` directory, if you are
using a *nix OS, or from within the `%TEMP%` directory, if you are running
Windows. This should be the same directory from where you are running OpenOCD.
It's very important that both `itmdump` and `openocd` are running from the same
directory!

```
# *nix
$ cd /tmp

# Windows
$ cd %TEMP%

$ itmdump itm.txt
```

This command will block as `itmdump` is now "watching" the `itm.txt` file. Leave
this terminal open.

Alright. Now, let's build the starter code and flash it into the
microcontroller.

```
$ xargo build --target thumbv7em-none-eabihf
```

Note that there's a `.gdbinit` at the root of the Cargo project. It's the same
one we used in the previous section.

```
$ arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/hello-world
Reading symbols from target/thumbv7em-none-eabihf/debug/hello-world...done.
(..)
Start address 0x8000194, load size 11682
Transfer rate: 18 KB/sec, 5841 bytes/write.
Breakpoint 1 at 0x80001e6: file $PWD/src/main.rs, line 10.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, hello_world::main () at $PWD/src/main.rs:10
10      pub fn main() -> ! {
```

Before we execute the `iprintln!` statement. We have to instruct OpenOCD to
redirect the ITM output into the same file that `itmdump` is watching.

```
(gdb) monitor tpiu config internal itm.txt uart off 8000000
```

All should be ready! Now execute the `iprintln!` statement.

```
(gdb) next
11          iprintln!("Hello, world!");

(gdb) next
13          loop {}
```

You should see some output in `itmdump`'s terminal:

```
# itmdump's terminal
Hello, world!
```

Awesome, right? Feel free to use `iprintln` as a logging tool in the coming
sections.

The `iprint!` macros are not the only thing that's wired to the ITM. `:-)`
