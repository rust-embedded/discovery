# USART

The microcontroller has a peripheral called USART, which stands for Universal
Synchronous/Asynchronous Receiver/Transmitter. This peripheral can be used to
implement several communication protocols like the "serial" communication
protocol.

Throughout this chapter, we'll use "serial" communication to exchange
information between the microcontroller and your laptop. But before we do that
we have to wire up everything.

I mentioned before that this protocol involves two data lines: TX and RX. TX
stands for transmitter and RX stands for receiver. Which is which depends from
which device we are looking the lines at.

We'll be using the pin `PA9` as the TX line and `PA10` as the RX line. IOW, the
pin `PA9` outputs data onto its wire wheres the pin `PA10` listens for data on
its wire.

We could have used a different pair of pins as the TX and RX pins. There's a
table in page 44 of the [Data Sheet] that list all the other possible pins we
could have used.

[Data Sheet]: http://www.st.com/resource/en/datasheet/stm32f303vc.pdf

The serial module also has TX and RX pins. We'll have to "cross" these pins that
is connect the microcontroller's TX pin to the serial module's RX pin and the
micro's RX pin to the serial module's TX pin. The wiring diagram below shows all
the necessary connections.

<p align="center">
<img height=640 title="F3 <-> Serial connection" src="assets/f3-serial.png">
</p>

These are the recommended steps to connect the microcontroller and the serial
module:

- Close OpenOCD and `itmdump`
- Disconnect the USB cables from the F3 and the serial module.
- Connect one of F3 GND pins to the GND pin of the serial module using a Female
  / Male (F/M) wire. Preferably, a black one.
- Connect the PA9 pin on the back of the F3 to the RXI pin of the serial module
  using a F/M wire.
- Connect the PA10 pin on the back of the F3 to the TXO pin of the serial
  module using a F/M wire.
- Now connect the USB cable to the F3.
- Finally connect the USB cable to the Serial module.
- Re-launch OpenOCD and `itmdump`

Everything's wired up! Let's proceed to send data back and forth.
