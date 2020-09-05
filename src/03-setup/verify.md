# Verify the installation

Let's verify that all the tools were installed correctly.

## Linux only

### Verify permissions

Connect the micro:bit to your computer using an USB cable.

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
$ ls -l /dev/bus/usb/003/004
crw-rw-rw-. 1 root root 189, 64 Sep  5 14:27 /dev/bus/usb/001/065
```

The permissions should be `crw-rw-rw-`. If it's not ... then check your [udev
rules] and try re-loading them with:

[udev rules]: linux.md#udev-rules

``` console
$ sudo udevadm control --reload-rules
```
