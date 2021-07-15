# Verify the installation

Let's verify that all the tools were installed correctly.

## Linux only

### Verify permissions

Connect the STM32F3DISCOVERY to your computer using an USB cable. Be sure to connect the cable to the "USB ST-LINK"
port, the USB port in the center of the edge of the board.

The STM32F3DISCOVERY should now appear as a USB device (file) in `/dev/bus/usb`. Let's find out how it got
enumerated:

``` console
lsusb | grep -i stm
```
This should result in:
``` console
$ lsusb | grep -i stm
Bus 003 Device 004: ID 0483:374b STMicroelectronics ST-LINK/V2.1
$ # ^^^        ^^^
```

In my case, the STM32F3DISCOVERY got connected to the bus #3 and got enumerated as the device #4. This means the
file `/dev/bus/usb/003/004` *is* the STM32F3DISCOVERY. Let's check its permissions:
``` console
$ ls -la /dev/bus/usb/003/004
crw-rw-rw-+ 1 root root 189, 259 Feb 28 13:32 /dev/bus/usb/003/00
```

The permissions should be `crw-rw-rw-`. If it's not ... then check your [udev
rules] and try re-loading them with:

[udev rules]: linux.md#udev-rules

``` console
sudo udevadm control --reload-rules
```

#### For older devices with OPTIONAL USB <-> FT232 based Serial Module

Unplug the STM32F3DISCOVERY and plug the Serial module. Now, figure out what's its associated file:

``` console
$ lsusb | grep -i ft232
Bus 003 Device 005: ID 0403:6001 Future Technology Devices International, Ltd FT232 Serial (UART) IC
```

In my case, it's the `/dev/bus/usb/003/005`. Now, check its permissions:

``` console
$ ls -l /dev/bus/usb/003/005
crw-rw-rw- 1 root root 189, 21 Sep 13 00:00 /dev/bus/usb/003/005
```

As before, the permissions should be `crw-rw-rw-`.

## Verify OpenOCD connection

Connect the STM32F3DISCOVERY using the USB cable to the USB port in the
center of edge of the board, the one that's labeled "USB ST-LINK".

Two *red* LEDs should turn on right after connecting the USB cable to the board.

> **IMPORTANT** There is more than one hardware revision of the STM32F3DISCOVERY board. For older
> revisions, you'll need to change the "interface" argument to `-f interface/stlink-v2.cfg` (note:
> no `-1` at the end). Alternatively, older revisions can use `-f board/stm32f3discovery.cfg`
> instead of `-f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`.

> **NOTE** OpenOCD v0.11.0 has deprecated `interface/stlink-v2.cfg` in favor of
> `interface/stlink.cfg` which supports ST-LINK/V1, ST-LINK/V2, ST-LINK/V2-1, and
> ST-LINK/V3.

### *Nix

> **FYI:** The `interface` directory is typically located in `/usr/share/openocd/scripts/`,
> which is the default location OpenOCD expects these files. If you've installed them
> somewhere else use the `-s /path/to/scripts/` option to specify your install directory.

``` console
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

or

``` console
openocd -f interface/stlink.cfg -f target/stm32f3x.cfg
```


### Windows

Below the references to `C:\OpenOCD` is the directory where OpenOCD is installed.

``` console
openocd -s C:\OpenOCD\share\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

> **NOTE** cygwin users have reported problems with the -s flag. If you run into
> that problem you can add `C:\OpenOCD\share\scripts\` directory to the parameters.

cygwin users:
``` console
openocd -f C:\OpenOCD\share\scripts\interface\stlink-v2-1.cfg -f C:\OpenOCD\share\scripts\target\stm32f3x.cfg
```

### All

OpenOCD is a service which forwards debug information from the ITM channel
to a file, `itm.txt`, as such it runs forever and does **not** return to the
terminal prompt.

The initial output of OpenOCD is something like:
``` console
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
Info : STLINK v2 JTAG v27 API v2 SWIM v15 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.915608
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

(If you don't ... then check the [general troubleshooting] instructions.)

[general troubleshooting]: ../appendix/1-general-troubleshooting/index.html

Also, one of the red LEDs, the one closest to the USB port, should start oscillating between red
light and green light.

That's it! It works. You can now use `Ctrl-c` to stop OpenOCD or close/kill the terminal.
