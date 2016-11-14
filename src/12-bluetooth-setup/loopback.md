# Loopback, again

After pairing your laptop to the Bluetooth module, your OS should have create a
device file / COM port for you. On Linux, it should be `/dev/rfcomm*`; on mac,
it should be `/dev/cu.*` and on Windows, it should be a new COM port.

We can now test the Bluetooth module with minicom/PuTTY. Because this module
doesn't have LED indicators for the transmission and reception events like the
serial module did, we'll test the module using a loopback connection:

<p align="center">
<img height=640 title="F3 <-> Bluetooth connection (loopback)" src="assets/f3-bluetooth-loopback.png">
</p>

Just connect the module's TXD pin to its RXD pin using a F/F wire.

Now, connect to the device using `minicom`/`PuTTY`:

```
$ minicom -D /dev/rfcomm0
```

Upon connecting, the blinking pattern of the Bluetooth module should change to:
long pause then blink twice quickly.

Typing inside minicom/PuTTY terminal should echo back what you type.
