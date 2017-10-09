# "Serial" communication

<a href="https://en.wikipedia.org/wiki/File:Serial_port.jpg">
<p align="center">
<img height="240" title="Standard serial port connector DE-9" src="https://upload.wikimedia.org/wikipedia/commons/thumb/e/ea/Serial_port.jpg/800px-Serial_port.jpg">
</p>
</a>

<p align="center">
<em>This is what we'll be using. I hope your laptop has one!</em>
</p>

Nah, don't worry. This connector, the DE-9, when out of fashion on PCs quite
some time ago; it got replaced by the Universal Serial Bus (USB). We won't be
dealing with the DE-9 connector itself but with the communication protocol that
this cable is/was usually used for.

So what's this "serial" communication? It's an *asynchronous* communication
protocol where two devices exchange data "serially", one bit at a time, using
two data lines (plus a common ground). The protocol is asynchronous in the sense
that neither of the data lines carries a clock signal. Instead both parties must
agree on how fast data will be sent along the wire *before* the communication
occurs. This protocol allows "duplex" communication as data can be sent from
A to B and from B to A simultaneously.

We'll be using this protocol to exchange data between the microcontroller and
your laptop. In contrast with the ITM protocol we have used before, with the
serial communication protocol we can send data from your laptop to the
microcontroller.

The next practical question you probably want to ask is: How fast can we send data
through this protocol?

This protocol works with frames. Each frame has one "start" bit, 5 to 9 bits of
payload and 0 to 2 "stop"" bits. The speed of the protocol is known as baud rate
and it's quoted in bits per second (bps). Common baud rates are: 9600, 19200,
38400, 57600 and 115200 bps.

To actually answer the question: With a common configuration of 1 start bit, 8
bits of payload, zero stop bits and a baud rate of 115200 bps one can, in
theory, send 12,800 frames of 9 bits per second. Since each one carries a byte
of information that results in a data rate of 12.8 KB/s. In practice, the data
rate will be lower because of processing times on the slower side of the
communication (the microcontroller).

Today's laptops/PCs don't support the serial communication protocol. So we can't
directly connect your laptop to the microcontroller. But that's where the serial
module comes in. This module will sit between the two and expose a "serial"
interface to the microcontroller and an USB interface to your laptop. The
microcontroller will see your laptop as another "serial" device and your laptop
will see the microcontroller as a virtual serial device.

Now, let's get familiar with the serial module and the serial communication
tools that your OS offers. Pick a route:

- [*nix](10-serial-communication/nix-tooling.html)
- [Windows](10-serial-communication/windows-tooling.html)
