# Loopbacks

We've tested sending data. It's time to test receiving it. Except that there's no other device that
can send us some data ... or is there?

Enter: loopbacks

<p align="center">
<img title="Serial module loopback" src="../assets/serial-loopback.png">
</p>

You can send data to yourself! Not very useful in production but very useful for debugging.

## Older board revision / external serial module

Connect the `TXO` and the `RXI` pins of the serial module together using a male to male jumper wire
as shown above.

Now enter some text into minicom/PuTTY and observe. What happens?

You should see three things:

- As before, the TX (red) LED blinks on each key press.
- But now the RX (green) LED blinks on each key press as well! This indicates that the serial module
  is receiving some data; the one it just sent.
- Finally, on the minicom/PuTTY console, you should see that what you type echoes back to the
  console.

## Newer board revision

If you have a newer revision of the board you can set up a loopback by shorting
the PC4 and PC5 pins using a female to female jumper wire, like [you did for the
SWO pin](../06-hello-world/index.html).

You should now be able to send data to yourself.

Now try to enter some text into minicom/PuTTY and observe.

> **NOTE**: To rule out the possibility of the existing firmware doing weird
> things to the serial pins (PC4 and PC5) we recommend *holding* the reset
> button while you enter text into minicom/PuTTY.

If all is working you should see what you type echoed back to minicom/PuTTY
console.

---

Now that you are familiar with sending and receiving data over serial port using minicom/PuTTY,
let's make your microcontroller and your computer talk!
