# Hello, world!

> **HEADS UP** Several readers have reported that the "solder bridge" SB10 (see back of the board)
> on the STM32F3DISCOVERY, which is required to use the ITM and the `iprint!` macros shown below, is
> **not** soldered even though the [User Manual][] (page 21) says that it **should be**.

> **TL;DR** You have two options to fix this: Either **solder** the solder bridge SB10 or connect a
> female to female jumper wire between SWO and PB3 as shown in the picture below.

[User Manual]: http://www.st.com/resource/en/user_manual/dm00063382.pdf

<p align="center">
<img height=640 title="Manual SWD connection" src="../assets/f3-swd.png">
</p>

---

Just a little more of helpful magic before we start doing low level stuff.

Blinking an LED is like the "Hello, world" of the embedded world.

But in this section, we'll run a proper "Hello, world" program that prints stuff to your computer
console.

Go to the `06-hello-world` directory. There's some starter code in it:

``` rust
{{#include src/main.rs}}
```

The `iprintln` macro will format messages and output them to the microcontroller's *ITM*. ITM stands
for Instrumentation Trace Macrocell and it's a communication protocol on top of SWD (Serial Wire
Debug) which can be used to send messages from the microcontroller to the debugging host. This
communication is only *one way*: the debugging host can't send data to the microcontroller.

OpenOCD, which is managing the debug session, can receive data sent through this ITM *channel* and
redirect it to a file.

The ITM protocol works with *frames* (you can think of them as Ethernet frames). Each frame has a
header and a variable length payload. OpenOCD will receive these frames and write them directly to a
file without parsing them. So, if the microntroller sends the string "Hello, world!" using the
`iprintln` macro, OpenOCD's output file won't exactly contain that string.

To retrieve the original string, OpenOCD's output file will have to be parsed. We'll use the
`itmdump` program to perform the parsing as new data arrives.

You should have already installed the `itmdump` program during the [installation chapter].

[installation chapter]: ../03-setup/index.html#itmdump

In a new terminal, run this command inside the `/tmp` directory, if you are using a \*nix OS, or from
within the `%TEMP%` directory, if you are running Windows. This should be the same directory from
where you are running OpenOCD.

> **NOTE** It's very important that both `itmdump` and `openocd` are running
from the same directory!

``` console
$ # itmdump terminal

$ # *nix
$ cd /tmp && touch itm.txt

$ # Windows
$ cd %TEMP% && type nul >> itm.txt

$ # both
$ itmdump -F -f itm.txt
```

This command will block as `itmdump` is now watching the `itm.txt` file. Leave this terminal open.

Make sure that F3 is connected to your computer. Open another terminal from `/tmp` directory (on Windows `%TEMP%`) to launch OpenOCD similar as described in chapter [First OpenOCD connection].

[First OpenOCD connection]: ../03-setup/verify.html#first-openocd-connection

Alright. Now, let's build the starter code and flash it into the microcontroller.

To avoid passing the `--target thumbv7em-none-eabihf` flag to every Cargo invocation we can set a
default target in .cargo/config:

``` diff
 [target.thumbv7em-none-eabihf]
 runner = "arm-none-eabi-gdb -q -x openocd.gdb"
 rustflags = [
   "-C", "link-arg=-Tlink.x",
 ]

+[build]
+target = "thumbv7em-none-eabihf"
```

Now if `--target` is not specified Cargo will assume that the target is `thumbv7em-none-eabihf`.

``` console
$ cargo run
Reading symbols from target/thumbv7em-none-eabihf/debug/hello-world...done.
(..)
Loading section .vector_table, size 0x400 lma 0x8000000
Loading section .text, size 0x27c4 lma 0x8000400
Loading section .rodata, size 0x744 lma 0x8002be0
Start address 0x8002980, load size 13064
Transfer rate: 18 KB/sec, 4354 bytes/write.
Breakpoint 1 at 0x8000402: file src/06-hello-world/src/main.rs, line 10.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, main () at src/06-hello-world/src/main.rs:10
10          let mut itm = aux6::init();
```

Note that there's a `openocd.gdb` at the root of the Cargo project. It's pretty similar to the one we
used in the previous section.

Before we execute the `iprintln!` statement. We have to instruct OpenOCD to redirect the ITM output
into the same file that `itmdump` is watching.

```
(gdb) # globally enable the ITM and redirect all output to itm.txt
(gdb) monitor tpiu config internal itm.txt uart off 8000000

(gdb) # enable the ITM port 0
(gdb) monitor itm port 0 on
```

All should be ready! Now execute the `iprintln!` statement.

```
(gdb) next
12          iprintln!(&mut itm.stim[0], "Hello, world!");

(gdb) next
14          loop {}
```

You should see some output in the `itmdump` terminal:

``` console
$ itmdump -F -f itm.txt
(..)
Hello, world!
```

Awesome, right? Feel free to use `iprintln` as a logging tool in the coming sections.

Next: That's not all! The `iprint!` macros are not the only thing that uses the ITM. `:-)`
