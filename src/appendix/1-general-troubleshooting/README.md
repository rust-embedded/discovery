# General troubleshooting

## OpenOCD problems

### can't connect to OpenOCD - "Error: open failed"

#### Symptoms

Upon trying to establish a *new connection* with the device you get an error
that looks like this:

```
$ openocd -f (..)
(..)
Error: open failed
in procedure 'init'
in procedure 'ocd_bouncer'
```

#### Cause + Fix

- All: The device is not (properly) connected. Check the USB connection using
  `lsusb` or the Device Manager.
- Linux: You may not have enough permission to open the device. Try again with
  `sudo`. If that works, you can use [these instructions] to make OpenOCD work
  without root privilege.
- Windows: You are probably missing the ST-LINK USB driver. Installation
  instructions [here].

[these instructions]: ../../03-setup/linux.md#udev-rules
[here]: ../../03-setup/windows.md#st-link-usb-driver

### can't connect to OpenOCD - "Polling again in X00ms"

#### Symptoms

Upon trying to establish a *new connection* with the device you get an error
that looks like this:

```
$ openocd -f (..)
(..)
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Previous state query failed, trying to reconnect
```

#### Cause

The microcontroller may have get stuck in some tight infinite loop or it may be
continuously raising an exception, e.g. the exception handler is raising an
exception.

#### Fix

- Close OpenOCD, if running
- Press and hold the reset (black) button
- Launch the OpenOCD command
- Now, release the reset button


### OpenOCD connection lost - "Polling again in X00ms"

#### Symptoms

A *running* OpenOCD session suddenly errors with:

```
# openocd -f (..)
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Previous state query failed, trying to reconnect
```

#### Cause

The USB connection was lost.

#### Fix

- Close OpenOCD
- Disconnect and re-connect the USB cable.
- Re-launch OpenOCD

### Can't flash the device - "Ignoring packet error, continuing..."

#### Symptoms

While flashing the device, you get:

```
$ arm-none-eabi-gdb $file
Start address 0x8000194, load size 31588
Transfer rate: 22 KB/sec, 5264 bytes/write.
Ignoring packet error, continuing...
Ignoring packet error, continuing...
```

#### Cause

Closed `itmdump` while a program that "printed" to the ITM was running. The
current GDB session will appear to work normally, just without ITM output but
the next GDB session will error with the message that was shown in the previous
section.

Or, `itmdump` was called **after** the `monitor tpiu` was issued thus making
`itmdump` delete the file / named-pipe that OpenOCD was writing to.

#### Fix

- Close/kill GDB, OpenOCD and `itmdump`
- Remove the file / named-pipe that `itmdump` was using (for example,
  `itm.txt`).
- Launch OpenOCD
- Then, launch `itmdump`
- Then, launch the GDB session that executes the `monitor tpiu` command.

## Cargo problems

### "can't find crate for `core`"

#### Symptoms

```
   Compiling volatile-register v0.1.2
   Compiling rlibc v1.0.0
   Compiling r0 v0.1.0
error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

Build failed, waiting for other jobs to finish...
Build failed, waiting for other jobs to finish...
error: Could not compile `r0`.

To learn more, run the command again with --verbose.
```

#### Cause

You are using a toolchain older than `nightly-2018-04-08` and forgot to call `rustup target add
thumbv7em-none-eabihf`.

#### Fix

Update your nightly and install the `thumbv7em-none-eabihf` target.

``` console
$ rustup update nightly

$ rustup target add thumbv7em-none-eabihf
```
