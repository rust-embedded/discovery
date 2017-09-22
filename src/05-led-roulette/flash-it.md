# Flash it

Flashing is the process of moving our program into the microcontroller's
(persistent) memory. Once flashed, the microcontroller will executed the flashed
program everytime it is powered on.

In this case, our `led-roulette` program will be the *only* program in the
microcontroller memory. By this I mean that there's nothing else running on the
microcontroller: no OS, no "daemon", nothing. `led-roulette` has full control
over the device.

Onto the actual flashing. First thing we need is to do is launch OpenOCD.
We did that in the previous section but this time we'll run the command inside a
temporary directory (`/tmp` on *nix; `%TEMP%` on Windows).

Make sure the F3 is connected to your laptop and run the following commands on a
new terminal.

```
# *nix
$ cd /tmp

# Windows
$ cd %TEMP%

# Windows: remember that you need an extra `-s %PATH_TO_OPENOCD%\share\scripts`
$ openocd \
  -f interface/stlink-v2-1.cfg \
  -f target/stm32f3x.cfg
```

> **NOTE** Older revisions need to pass slightly different arguments to
> `openocd`. Review [this section] for the details.

[this section]: 03-setup/verify.html#First%20OpenOCD%20connection

The program will block; leave that terminal open.

Now it's a good time to explain what this command is actually doing.

I mentioned that the F3 actually has two microcontrollers. One of them is used
as a programmer/debugger. The part of the board that's used as a programmer is
called ST-LINK (that's how STMicroelectronics decided to call it). This
"ST-LINK" is connected to the target microcontroller using a Serial Wire Debug
(SWD) interface (this interface is an ARM standard so you'll run into it when
dealing with other Cortex-M based microcontrollers). This SWD interface can be
used to flash and debug a microcontroller. The ST-LINK is connected to the
"USB ST-LINK" port and will appear as an USB device when you connect the F3 to
your laptop.

<p align="center">
<img height=640 title="On-board ST-LINK" src="assets/st-link.png">
</p>


As for OpenOCD. It's a software that provides some services like a *GDB server*
"on top" of USB devices that expose a debugging protocol like SWD or JTAG.

Onto the actual command: Those `.cfg` files we are using instruct OpenOCD to
look for a ST-LINK USB device (`interface/stlink-v2-1.cfg`) and to expect a
STM32F3XX microcontroller (`target/stm32f3x.cfg`) to be connected to the
ST-LINK.

The OpenOCD output looks like this:

```
Open On-Chip Debugger 0.9.0 (2016-04-27-23:18)
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
Info : STLINK v2 JTAG v27 API v2 SWIM v15 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.919073
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

The "6 breakpoints, 4 watchpoints" part indicates the debugging features the
processor has available.

I mentioned that OpenOCD provides a GDB server so let's connect to that right
now:

```
$ arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...done.
(gdb)
```

This only opens a GDB shell. To actually connect to the OpenOCD GDB server, use
the following command within the GDB shell:

```
(gdb) target remote :3333
Remote debugging using :3333
0x00000000 in ?? ()
```

OpenOCD's GDB server is listening on TCP port 3333 (localhost). This command is
connecting to that port.

After entering this command, you'll see new output in the OpenOCD terminal:

``` diff
 Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
+Info : accepting 'gdb' connection on tcp/3333
+Info : device id = 0x10036422
+Info : flash size = 256kbytes
```

Almost there. To flash the device, we'll use the `load` command inside the GDB
shell:

```
(gdb) load
Loading section .text, size 0x6798 lma 0x8000000
Loading section .ARM.extab.text._ZN44_$LT$char$u20$as$u20$core..char..CharExt$GT$11encode_utf817h4f3134c02513b5e1E, size 0xc lma 0x8006798
Loading section .ARM.extab.text._ZN4core3fmt9Formatter11debug_tuple17hf0ed23ebdee33c00E, size 0xc lma 0x80067a4
Start address 0x8000194, load size 26544
Transfer rate: 21 KB/sec, 6636 bytes/write.
```

And that's it. You'll also see new output in the OpenOCD terminal.

``` diff
 Info : flash size = 256kbytes
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+adapter speed: 950 kHz
+target state: halted
+target halted due to debug-request, current mode: Thread
+xPSR: 0x01000000 pc: 0x08000194 msp: 0x2000a000
+Info : Unable to match requested speed 8000 kHz, using 4000 kHz
+Info : Unable to match requested speed 8000 kHz, using 4000 kHz
+adapter speed: 4000 kHz
+target state: halted
+target halted due to breakpoint, current mode: Thread
+xPSR: 0x61000000 pc: 0x2000003a msp: 0x2000a000
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+adapter speed: 950 kHz
+target state: halted
+target halted due to debug-request, current mode: Thread
+xPSR: 0x01000000 pc: 0x08000194 msp: 0x2000a000
```

Our program is loaded, let's debug it!
