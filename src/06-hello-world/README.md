# Hello, world!

> **HEADS UP** The "solder bridge" SB10 (see back of the board) on the STM32F3DISCOVERY, which is
> required to use the ITM and the `iprint!` macros shown below, is **not** soldered by default
> (see page 21 of the [User Manual][]).

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

Make sure that the STM32F3DISCOVERY board is connected to your computer. Open another terminal
from `/tmp` directory (on Windows `%TEMP%`) to launch OpenOCD similar as described in [chapter 3].

[chapter 3]: ../03-setup/verify.html#first-openocd-connection

Alright. Now, let's build the starter code and flash it into the microcontroller.

To avoid passing the `--target thumbv7em-none-eabihf` flag to every Cargo invocation we
have added `[build]` with a default target, `target = "thumbv7em-none-eabihf"`, to .cargo/config.
Now if `--target` is not specified `cargo` will assume that the target is `thumbv7em-none-eabihf`.

```
[target.thumbv7em-none-eabihf]
runner = "arm-none-eabi-gdb -q -x openocd.gdb"
rustflags = [
  "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv7em-none-eabihf"
```

In addition, our `opendocd.gdb` has some additional lines. Compared to the previous
section `set`'s and initialize `monitor`ing so `iprint!` and `iprintln!`
macros work and output is visible on a console. Below the contents with comments:

``` console
$ cat openocd.gdb
# Connect to gdb remote server
target remote :3333

# Load will flash the code
load

# Enable demangling asm names on disassembly
set print asm-demangle on

# Enable pretty printing
set print pretty on

# Disable style sources as the default colors can be hard to read
set style sources off

# Have the tpiu send the data to a file itm.txt
monitor tpiu config internal itm.txt uart off 8000000

# Turn on the itm port
monitor itm port 0 on

# Set a breakpoint at main
break main

# Continue running and we'll hit the main breakpoint
continue
```

We will now run the application and single step through it. Since we've added
the `monitor` commands in `openocd.gdb` OpenOCD will redirect the ITM output to
itm.txt and `itmdump` will write it to its terminal window.

``` console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `arm-none-eabi-gdb -q -x openocd.gdb ~/prgs/rust/tutorial/embedded-discovery/target/thumbv7em-none-eabihf/debug/hello-world`
Reading symbols from ~/prgs/rust/tutorial/embedded-discovery/target/thumbv7em-none-eabihf/debug/hello-world...
0x00000000 in ?? ()
Loading section .vector_table, size 0x194 lma 0x8000000
Loading section .text, size 0x28d8 lma 0x8000194
Loading section .rodata, size 0x6b8 lma 0x8002a6c
Start address 0x08000194, load size 12580
Transfer rate: 18 KB/sec, 4193 bytes/write.
Breakpoint 1 at 0x80001f0: file src/06-hello-world/src/main.rs, line 8.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, hello_world::__cortex_m_rt_main_trampoline () at src/06-hello-world/src/main.rs:8
8       #[entry]
```

We are now stopped at `#[entry]` and since, as before, it's a trampoline:

``` console
(gdb) disassemble /m
Dump of assembler code for function main:
8	#[entry]
   0x080001ec <+0>:	push	{r7, lr}
   0x080001ee <+2>:	mov	r7, sp
=> 0x080001f0 <+4>:	bl	0x80001f6 <hello_world::__cortex_m_rt_main>
   0x080001f4 <+8>:	udf	#254	; 0xfe
```

We need to initially `step` into the main function which will position us at line 10:

``` text
(gdb) step
hello_world::__cortex_m_rt_main () at src/06-hello-world/src/main.rs:10
10	    let mut itm = aux6::init();
```

Now issue a `next` command which will exectue `aux6::init()` and
stop at he next executable statement in `main.rs`, which
positions us at line 12:

``` text
(gdb) next
12	    iprintln!(&mut itm.stim[0], "Hello, world!");
```

Then issue another `next` command which will execute
line 12, executing the `iprintln` and stop at line 14:

``` text
(gdb) next
14	    loop {}
```

Now since `iprintln` has been executed the output on the `itmdump`
terminal window should be the `Hello, world!` string:

``` console
$ itmdump -F -f itm.txt
(...)
Hello, world!
```

Awesome, right? Feel free to use `iprintln` as a logging tool in the coming sections.

Next: That's not all! The `iprint!` macros are not the only thing that uses the ITM. `:-)`
