## AT commands

> **NOTE** incomplete

<p align="center">
<img height=640 title="Bluetooth <-> Serial connection" src="assets/bluetooth-serial.png">
</p>

Entering AT mode:

- Power off the Bluetooth module
- Press and hold the button on the Bluetooth module
- Power on the Bluetooth module
- Now, release the button

> **TODO** blinking pattern

The AT mode always operates at a baud rate of 38,400.

Commands (via minicom)

- Sanity check

```
$ at
OK
OK
(..)
```

Answers `OK` repeatedly until you hit ENTER again.

- Rename

```
$ at+name=ferris
OK
```

- Change the baud rate

```
$ at+uart=115200,0,0
OK
```
