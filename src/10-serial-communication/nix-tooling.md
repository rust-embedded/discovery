# *nix tooling

Connect the serial module to your laptop and let's find out what name the OS
assigned to it.

```
$ dmesg | grep -i tty
(..)
[  +0.000155] usb 3-2: FTDI USB Serial Device converter now attached to ttyUSB0
```

> **NOTE** On macs, the USB device will named like this: `cu.usbserial-*`.
> Adjust the following commands accordingly!

But what's this `ttyUSB0` thing? It's a file of course! Everything is a file in
*nix:

```
$ ls -l /dev/ttyUSB0
crw-rw---- 1 root uucp 188, 0 Oct 27 00:00 /dev/ttyUSB0
```

You can send out data by simply writing to this file:

```
$ echo 'Hello, world!' > /dev/ttyUSB0
```

You should see the TX (red) LED on the serial module blink, just once and very
fast!

## minicom

Dealing with serial devices using `echo` is far from ergonomic. So, we'll use
the program `minicom` to interact with the serial device using the keyboard.

We must configure `minicom` before we use it. There are quite a few ways to do
that but we'll use a `.minirc.dfl` file in the home directory. Create a file
in `~/.minirc.dfl` with the following contents:

```
pu baudrate 115200
pu bits 8
pu parity N
pu stopbits 1
pu rtscts No
pu xonxoff No
```

> **NOTE** Make sure this file ends in a newline! Otherwise, `minicom` will fail
> to read it.

That file should be straightforward to read (except for the last two lines), but
nonetheless let's go over it line by line:

- `pu baudrate 115200`. Sets baud rate to 115200.
- `pu bits 8`. 8 bits per frame.
- `pu parity N`. No parity check.
- `pu stopbits 1`. 1 stop bit.
- `pu rtscts No`. No hardware control flow.
- `pu xonxoff No`. No software control flow.

Once that's in place. We can launch `minicom`

```
$ minicom -D /dev/ttyUSB0 -b 115200
```

This tells `minicom` to "open" the serial device at `/dev/ttyUSB0` and set its
baud rate to 115200. A text-based user interface (TUI) will pop out.

<p align="center">
<img height="480" title="minicom" src="assets/minicom.png">
</p>

You can now send data using the keyboard! Go ahead and type something. Note that
the TUI *won't* echo back what you type but you'll see TX (red) LED on the
serial module blink with each keystroke.

## `minicom` commands

`minicom` exposes commands via keyboard shortcuts. On Linux, the shortcuts start
with `Ctrl+A`. On mac, the shortcuts start with the `Meta` key. Some useful
commands below:

- `Ctrl+A` + `Z`. Minicom Command Summary
- `Ctrl+A` + `C`. Clear the screen
- `Ctrl+A` + `X`. Exit and reset
- `Ctrl+A` + `Q`. Quit with no reset

> **NOTE** mac users: In the above commands, replace `Ctrl+A` with `Meta`.
