# Verify the installation

Let's verify that all the tools were installed correctly.

## Linux only

### Verify permissions

Connect the micro:bit to your computer using a USB cable.

The micro:bit should now appear as a USB device (file) in `/dev/bus/usb`. Let's find out how it got
enumerated:

``` console
$ lsusb | grep -i "NXP ARM mbed"
Bus 001 Device 065: ID 0d28:0204 NXP ARM mbed
$ # ^^^        ^^^
```

In my case, the micro:bit got connected to the bus #1 and got enumerated as the device #65. This means the
file `/dev/bus/usb/001/065` *is* the micro:bit. Let's check its permissions:

``` console
$ ls -l /dev/bus/usb/001/065
crw-rw-rw-. 1 root root 189, 64 Sep  5 14:27 /dev/bus/usb/001/065
```

The permissions should be `crw-rw-rw-`. If it's not ... then check your [udev
rules] and try re-loading them with:

[udev rules]: linux.md#udev-rules

``` console
$ sudo udevadm control --reload-rules
```

# All

## Verifying cargo-embed
First, connect the micro:bit to your Computer using a USB cable.

At least an orange LED right next to the USB port of the micro:bit should light up.
Furthermore, if you have never flashed another program on to your micro:bit, the default
program the micro:bit ships with should start blinking the red LEDs on its back, you
can ignore them.

Next up you will have to modify `Embed.toml` in the `src/03-setup` directory of the
book's source code. In the `default.general` section you will find two commented out
chip variants:

```toml
[default.general]
# chip = "nrf52833_xxAA" # uncomment this line for micro:bit V2
# chip = "nrf51822_xxAA" # uncomment this line for micro:bit V1
```

If you are working with the micro:bit v2 board uncomment the first, for the v1
uncomment the second line.

Next run one of these commands:

```
$ # make sure you are in src/03-setup of the books source code
$ # If you are working with micro:bit v2
$ rustup target add thumbv7em-none-eabihf
$ cargo embed --target thumbv7em-none-eabihf

$ # If you are working with micro:bit v1
$ rustup target add thumbv6m-none-eabi
$ cargo embed --target thumbv6m-none-eabi
```

If everything works correctly cargo-embed should first compile the small example program
in this directory, then flash it and finally open a nice text based user interface that
prints Hello World.

(If it does not, check out [general troubleshooting] instructions.)

[general troubleshooting]: ../appendix/1-general-troubleshooting/index.html

This output is coming from the small Rust program you just flashed on to your micro:bit.
Everything is working properly and you can continue with the next chapters!
