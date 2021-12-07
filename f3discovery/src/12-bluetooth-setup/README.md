# Bluetooth setup

It's time to get rid of some wires. Serial communication can not only be emulated on top of the USB
protocol; it can also be emulated on top of the Bluetooth protocol. This serial over Bluetooth
protocol is known as RFCOMM.

Before we use the Bluetooth module with the microcontroller, let's first interact with it using
minicom/PuTTY.

The first thing we'll need to do is: turn on the Bluetooth module. We'll have to share some of the
F3 power to it using the following connection:

<p align="center">
<img height=640 title="F3 <-> Bluetooth connection (power only)" src="../assets/f3-bluetooth-power-only.png">
</p>

The recommend steps to wire this up are:

- Close OpenOCD and `itmdump`
- Disconnect the USB cables from the F3 and the serial module.
- Connect F3's GND pin to the Bluetooth's GND pin using a female to female (F/F) wire. Preferably, a
  black one.
- Connect F3's 5V pin to the Bluetooth's VCC pin using a F/F wire. Preferably, a red one.
- Then, connect the USB cable back to the F3.
- Re-launch OpenOCD and `itmdump`

Two LEDs, a blue one and a red one, on the Bluetooth module should start blinking right after you
power on the F3 board.

Next thing to do is pair your computer and the Bluetooth module. AFAIK, Windows and mac users can
simply use their OS default Bluetooth manager to do the pairing. The Bluetooth module default pin
is 1234.

Linux users will have to follow (some of) [these instructions].

[these instructions]: linux.md
