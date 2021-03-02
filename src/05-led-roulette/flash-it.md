# Flash it

Flashing is the process of moving our program into the microcontroller's (persistent) memory. Once
flashed, the microcontroller will execute the flashed program every time it is powered on.

In this case, our `led-roulette` program will be the *only* program in the microcontroller memory.
By this I mean that there's nothing else running on the microcontroller: no OS, no "daemon",
nothing. `led-roulette` has full control over the device.

Onto the actual flashing. First thing we need is to do is launch OpenOCD. We did that in the
previous section but this time we'll run the command inside a temporary directory (`/tmp` on \*nix;
`%TEMP%` on Windows).

Make sure the F3 is connected to your computer and run the following commands on a **new terminal**.

## For *nix & MacOS:
``` console
cd /tmp
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

## For Windows **Note**: substitute `C:` for the actual OpenOCD path:
```
cd %TEMP%
openocd -s C:\share\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

> **NOTE** Older revisions of the board need to pass slightly different arguments to
> `openocd`. Review [this section] for the details.

[this section]: ../03-setup/verify.md#first-openocd-connection

The program will block; leave that terminal open.

Now it's a good time to explain what the `openocd` command is actually doing.

I mentioned that the STM32F3DISCOVERY (aka F3) actually has two microcontrollers. One of them is used as a
programmer/debugger. The part of the board that's used as a programmer is called ST-LINK (that's what
STMicroelectronics decided to call it). This ST-LINK is connected to the target microcontroller
using a Serial Wire Debug (SWD) interface (this interface is an ARM standard so you'll run into it
when dealing with other Cortex-M based microcontrollers). This SWD interface can be used to flash
and debug a microcontroller. The ST-LINK is connected to the "USB ST-LINK" port and will appear as
a USB device when you connect the F3 to your computer.

<p align="center">
<img height=640 title="On-board ST-LINK" src="../assets/st-link.png">
</p>


As for OpenOCD, it's software that provides some services like a *GDB server* on top of USB
devices that expose a debugging protocol like SWD or JTAG.

Onto the actual command: those `.cfg` files we are using instruct OpenOCD to look for a ST-LINK USB
device (`interface/stlink-v2-1.cfg`) and to expect a STM32F3XX microcontroller
(`target/stm32f3x.cfg`) to be connected to the ST-LINK.

The OpenOCD output looks like this:
``` console
$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
none separate
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v37 API v2 SWIM v26 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.888183
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

The "6 breakpoints, 4 watchpoints" part indicates the debugging features the processor has
available.

Leave that `openocd` process running, and in the previous terminal or a new terminal
**make sure that you are inside the project's `src/05-led-roulette/` directory**.

I mentioned that OpenOCD provides a GDB server so let's connect to that right now:

## Execute GDB

First we need to determine what version of GDB you have that is capable of debugging ARM binaries.

This could be any one of the commands below, try each one:
``` console
arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette
```
``` console
gdb-multiarch -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette
```
``` console
gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette
```
### **Failing case**

You can detect a failing case if there is a `warning` or `error` after the `Remote debugging using :3333` line:
```
$ gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...
Remote debugging using :3333
warning: Architecture rejected target-supplied description
Truncated register 16 in remote 'g' packet
(gdb)
```
### **Successful case**
Successful case 1:
```
$ arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...
Remote debugging using :3333
cortex_m_rt::Reset () at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:497
497     pub unsafe extern "C" fn Reset() -> ! {
(gdb)
```
Successful case 2:
```
~/embedded-discovery/src/05-led-roulette (master)
$ arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...
Remote debugging using :3333
0x00000000 in ?? ()
(gdb)
```
In both failing and successful cases you should see new output in the **OpenOCD terminal**, something like the following:
``` diff
 Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
+Info : accepting 'gdb' connection on tcp/3333
+Info : device id = 0x10036422
+Info : flash size = 256kbytes
```

**NOTE**: If you are getting an error like `undefined debug reason 7 - target needs reset`, you can try running `monitor reset halt` as described [here](https://stackoverflow.com/questions/38994596/reason-7-target-needs-reset-unreliable-debugging-setup).

By default OpenOCD's GDB server listens on TCP port 3333 (localhost). This command is connecting to
that port.

## Update .cargo/config

Now that you've successfully determined which debugger you need to use
we need to change `.cargo/config` so that `cargo run` command can succeed.

Get back to the terminal prompt and looking at `.cargo/config`:
``` console
$ cat .cargo/config
[target.thumbv7em-none-eabihf]
runner = "arm-none-eabi-gdb -q"
rustflags = [
  "-C", "link-arg=-Tlink.x",
]

```
Use your favorite editor to edit `.cargo/config` so that the
runner line contains the name of that debugger:
``` console
nano .cargo/config
```
For example, if your debugger was `gdb-multiarch` then after
editing you should have:
``` console
$ cat .cargo/config
[target.thumbv7em-none-eabihf]
runner = "gdb-mulitarch -q"
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```
And `git diff` should be:
``` diff
$ git diff .cargo/config
diff --git a/src/05-led-roulette/.cargo/config b/src/05-led-roulette/.cargo/config
index 01d25c8..c23dc80 100644
--- a/src/05-led-roulette/.cargo/config
+++ b/src/05-led-roulette/.cargo/config
@@ -1,5 +1,5 @@
 [target.thumbv7em-none-eabihf]
-runner = "arm-none-eabi-gdb -q"
+runner = "gdb-multiarch -q"
 rustflags = [
   "-C", "link-arg=-Tlink.x",
 ]
```

Now that you have `.cargo/config` setup to let's test it and use `cargo run` to
start the debug session:
```
~/embedded-discovery/src/05-led-roulette
$ cargo run --target thumbv7em-none-eabihf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `arm-none-eabi-gdb -q ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette`
Reading symbols from ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette...

(gdb) target remote :3333
Remote debugging using :3333
0x00000000 in ?? ()

(gdb)
```

Bravo, you'll be making additional changes to `.cargo/config` in future
sections to make building and debugging easier.

> **Note** the default `.cargo/config` in every chapter assumes
> the debugger is `arm-none-eabi-gdb`. So the first thing you should
> do when you start a new chapter is edit `.cargo/config`!

## Flash the device

Assuming you have gdb running, if not start it as suggested in the previous section.

Now use the `load` command in `gdb` to actually flash the program into the device:
```
(gdb) load
Loading section .vector_table, size 0x194 lma 0x8000000
Loading section .text, size 0x20ec lma 0x8000194
Loading section .rodata, size 0x514 lma 0x8002280
Start address 0x08000194, load size 10132
Transfer rate: 17 KB/sec, 3377 bytes/write.
```

You'll also see new output in the OpenOCD terminal, something like:

``` diff
 Info : flash size = 256kbytes
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+adapter speed: 950 kHz
+target halted due to debug-request, current mode: Thread
+xPSR: 0x01000000 pc: 0x08000194 msp: 0x2000a000
+Info : Unable to match requested speed 8000 kHz, using 4000 kHz
+Info : Unable to match requested speed 8000 kHz, using 4000 kHz
+adapter speed: 4000 kHz
+target halted due to breakpoint, current mode: Thread
+xPSR: 0x61000000 pc: 0x2000003a msp: 0x2000a000
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+adapter speed: 950 kHz
+target halted due to debug-request, current mode: Thread
+xPSR: 0x01000000 pc: 0x08000194 msp: 0x2000a000
```

Our program is loaded, let's debug it!
