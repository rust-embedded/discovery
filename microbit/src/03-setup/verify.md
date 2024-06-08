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
file `/dev/bus/usb/001/065` *is* the micro:bit. Let's check the file permissions:

``` console
$ ls -l /dev/bus/usb/001/065
crw-rw-r--+ 1 nobody nobody 189, 64 Sep  5 14:27 /dev/bus/usb/001/065
```

The permissions should be `crw-rw-r--+`, note the `+` at the end, then see your access rights by running the following command.

``` console
$ getfacl /dev/bus/usb/001/065
getfacl: Removing leadin '/' from absolute path names
# file: dev/bus/usb/001/065
# owner: nobody
# group: nobody
user::rw-
user:<YOUR-USER-NAME>:rw-
group::rw-
mask::rw-
other::r-
```

You should see your username in the list above with the `rw-` permissions, if not ... then check your [udev
rules] and try re-loading them with:

[udev rules]: linux.md#udev-rules

``` console
$ sudo udevadm control --reload
$ sudo udevadm trigger
```

# All

## Verifying cargo-embed
First, connect the micro:bit to your Computer using a USB cable.

At least an orange LED right next to the USB port of the micro:bit should light up.
Furthermore, if you have never flashed another program on to your micro:bit, the default
program the micro:bit ships with should start blinking the red LEDs on its back, you
can ignore them.

Now let's see if probe-rs, and by extensions cargo-embed can see your micro:bit, you can do this by running the following command.

``` console
$ probe-rs list
The following debug probes were found:
[0]: BBC micro:bit CMSIS-DAP -- 0d28:0204:990636020005282030f57fa14252d446000000006e052820 (CMSIS-DAP)
```

Or if you want more information about the micro:bits debug capabilities then you can run:

``` console
$ probe-rs info
Probing target via JTAG

Error identifying target using protocol JTAG: The probe does not support the JTAG protocol.

Probing target via SWD

ARM Chip with debug port Default:
Debug Port: DPv1, DP Designer: ARM Ltd
├── 0 MemoryAP
│   └── ROM Table (Class 1), Designer: Nordic VLSI ASA
│       ├── Cortex-M4 SCS   (Generic IP component)
│       │   └── CPUID
│       │       ├── IMPLEMENTER: ARM Ltd
│       │       ├── VARIANT: 0
│       │       ├── PARTNO: Cortex-M4
│       │       └── REVISION: 1
│       ├── Cortex-M3 DWT   (Generic IP component)
│       ├── Cortex-M3 FBP   (Generic IP component)
│       ├── Cortex-M3 ITM   (Generic IP component)
│       ├── Cortex-M4 TPIU  (Coresight Component)
│       └── Cortex-M4 ETM   (Coresight Component)
└── 1 Unknown AP (Designer: Nordic VLSI ASA, Class: Undefined, Type: 0x0, Variant: 0x0, Revision: 0x0)


Debugging RISC-V targets over SWD is not supported. For these targets, JTAG is the only supported protocol. RISC-V specific information cannot be printed.
Debugging Xtensa targets over SWD is not supported. For these targets, JTAG is the only supported protocol. Xtensa specific information cannot be printed.

```

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
