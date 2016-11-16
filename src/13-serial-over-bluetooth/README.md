# Serial over Bluetooth

Now that we verify that the Bluetooth module works with minicom/PuTTY, let's
connect it to the microcontroller:

<p align="center">
<img height=640 title="F3 <-> Bluetooth connection" src="assets/f3-bluetooth.png">
</p>

Recommended steps to wire this up:

- Close OpenOCD and `itmdump`.
- Disconnect the F3 from your laptop.
- Connect F3's GND pin to the module's GND pin using a Female/Female (F/F) wire
  (preferably, a black one).
- Connect F3's 5V pin to the module's VCC pin using a F/F wire (preferably, a
  red one).
- Connect the PA9 (TX) pin on the back of the F3 to the Bluetooth's RXD pin
  using a F/F wire.
- Connect the PA10 (RX) pin on the back of the F3 to the Bluetooth's TXD pin
  using a F/F wire.
- Now connect the F3 and your laptop using an USB cable.
- Re-launch OpenOCD and `itmdump`.

And that's it! You should be able to run all the programs you wrote in [section
11] without modification! Just make sure you open the right serial device / COM
port.

[section 10]: 11-usart/README.html
