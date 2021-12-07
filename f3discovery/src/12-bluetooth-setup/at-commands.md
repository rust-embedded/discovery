## AT commands

The Bluetooth module and the F3 need to be configured to communicate at the same baud rate. The tutorial code initializes the UART1 serial device to a baud rate of 115200. The HC-05 Bluetooth module is configured at a baud rate of 9600 by default.

The Bluetooth module supports an AT mode that allows you to examine and change its configuration and settings. To utilize the AT mode, connect the Bluetooth module to the F3 and FTDI as shown in the following diagram.

<p align="center">
<img height=640 title="Bluetooth <-> Serial connection" src="../assets/bluetooth-serial.png">
</p>

Recommended steps to enter AT mode:

- Disconnect the F3 and FTDI from your computer.
- Connect F3's GND pin to the Bluetooth's GND pin using a Female/Female (F/F) wire
  (preferably, a black one).
- Connect F3's 5V pin to the Bluetooth's VCC pin using a F/F wire (preferably, a
  red one).
- Connect the FTDI RXI pin to the Bluetooth's TXD pin using a Female/Male (F/M) wire.
- Connect the FTDI TXO pin to the Bluetooth's RXD pin using a Female/Male (F/M) wire.
- Now connect the FTDI to your computer via USB cable.
- Next connect the F3 to your computer via USB cable while simultaneously pressing and holding the button on the Bluetooth module (kinda tricky).
- Now, release the button and the Bluetooth module will enter AT mode. You can confirm this by observing that the red LED on the Bluetooth module is blinking in a slow pattern (approx 1-2 seconds on/off).

The AT mode always operates at a baud rate of 38400, so configure your terminal program for that baud rate and connect to the FTDI device.

When your serial connection is established, you may get a bunch of `ERROR: (0)` repeatedly being displayed. If this happens, just hit ENTER to stop the errors.

### Sanity check

```
$ at
OK
OK
(etc...)
```

Answers `OK` repeatedly until you hit ENTER again.

### Rename the device

```
$ at+name=ferris
OK
```

### Query for the current baud rate of the Bluetooth module

```
at+uart?
+UART:9600,0,0
OK
+UART:9600,0,0
OK
(etc ...)
```

### Change the baud rate

```
$ at+uart=115200,0,0
OK
```
