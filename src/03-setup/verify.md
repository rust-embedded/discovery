# Verify the installation

Let's verify that all the tools were installed correctly.

## Linux only

### Verify permissions

Connect the F3 to your laptop using an USB cable. Be sure to connect the cable
to the "USB ST-LINK" port, the USB port at the middle.

The F3 should now appear as a USB device (file) in `/dev/bus/usb`. Let's find
out how it got enumerated:

```
$ lsusb | grep -i stm
Bus 003 Device 004: ID 0483:374b STMicroelectronics ST-LINK/V2.1
    ^^^        ^^^
```

In my case, the F3 got connected to the bus #3 and got enumerated as the device
#4. This means the file `/dev/bus/usb/003/004` *is* the F3. Let's check its
permissions:

```
$ ls -l /dev/bus/usb/003/004
crw-rw-r-- 1 root uucp 189, 262 Oct 27 00:00 /dev/bus/usb/003/004
```

The group should be `uucp`. If it's not ... then check your [udev rules] and try
re-loading them with:

[udev rules]: 03-setup/linux.html#udev%20rules

```
$ sudo udevadm control --reload-rules
```

Now let's repeat the procedure for the Serial module.

Unplug the F3 and plug the Serial module. Now, figure out what's its associated
file:

```
$ lsusb | grep -i ft232
Bus 003 Device 005: ID 0403:6001 Future Technology Devices International, Ltd FT232 Serial (UART) IC
```

In my case, it's the `/dev/bus/usb/003/005`. Now, check its permissions:

```
$ ls -l /dev/bus/usb/003/005
crw-rw-r--+ 1 root uucp 189, 261 Oct 27 00:00 /dev/bus/usb/003/005
```

As before, the group should be `uucp`.

## All

### First OpenOCD connection

First, connect the F3 to your laptop using an USB cable. Connect the cable to
USB port at the center of the F3, the one that's labeled "USB ST-LINK".

Two *red* LEDs should turn on right after connecting the USB cable to the board.

Next, call this command:

```
# *nix
$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg

# Windows
$ openocd -s C:\OpenOCD\share\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

> **NOTE** Windows users: `C:\OpenOCD` is the directory where you installed
> OpenOCD to.

> **IMPORTANT** There is more than one hardware revision of the
> STM32F3DISCOVERY board. For older revisions, you'll need to change the
> "interface" argument to `-f interface/stlink-v2.cfg` (note: no `-1` at the
> end). Alternatively, older revisions can use `-f board/stm32f3discovery.cfg`
> instead of `-f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`.

You should see output like this:

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
Info : Target voltage: 2.914184
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

(If you don't ... then check the [general troubleshooting] instructions.)

[general troubleshooting]: appendix/1-general-troubleshooting/README.html

And `openocd` will block. That's fine.

Also, one of the red LEDs, the one closest to the USB port, should start
oscillating between red light and green light.

That's it! It works. You can now close/kill `openocd`.
