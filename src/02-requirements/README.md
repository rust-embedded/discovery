# Hardware/knowledge requirements

The only "knowledge requirement" to read this book is to know "some" Rust. It's
hard for me to quantify "some" but at least I can tell you that you don't need
to grok generics but you do need to know how to use closures.

Also, to follow this material you'll need the following hardware:

(Some components are optional but recommended)

- A [STM32F3DISCOVERY] board.

[STM32F3DISCOVERY]: http://www.st.com/en/evaluation-tools/stm32f3discovery.html

<p align="center">
<img title="STM32F3DISCOVERY" src="assets/f3.jpg">
</p>

- OPTIONAL. A **3.3V** USB <-> Serial module. [This particular model][sparkfun]
  will be used throughout this material but you can use any other model as long
  as it operates at 3.3V.

[sparkfun]: https://www.sparkfun.com/products/9873

<p align="center">
<img title="A 3.3v USB <-> Serial module" src="assets/serial.jpg">
</p>

- OPTIONAL. A HC-05 Bluetooth module (with headers!). A HC-06 would work too.

<p align="center">
<img title="The HC-05 Bluetooth module" src="assets/bluetooth.jpg">
</p>

- Two mini-B USB cables. One is required to make the STM32F3DISCOVERY board
  work. The other is only required if you have the Serial <-> USB module.

<p align="center">
<img title="mini-B USB cable" src="assets/usb-cable.jpg">
</p>

> **NOTE** These are **not** the USB cables that ship with pretty much every
> Android phone; those are *micro* USB cables. Make sure you have the right
> thing!

- OPTIONAL. 4 Female/Female, 4 Male/Female and 1 Male/Male jumper wires. Only if
  you'll be using the USB <-> Serial and Bluetooth modules.

<p align="center">
<img title="Jumper wires" src="assets/jumper-wires.jpg">
</p>

> **FAQ**: Wait, why do I need this specific hardware?

It makes my life and yours much easier.

The material is much, much more approachable if we don't have to worry about
hardware differences. Trust me on this one.

> **FAQ**: Can I follow this material with a different development board?

Maybe? It depends mainly on two things: your previous experience with
microcontrollers and/or whether there already exists a high level crate, like
the [f3], for your development board somewhere.

Regardless, with a different development board, this text would lost most if not
all its beginner friendliness and "easy to follow"-ness, IMO.

If you have a different development board and you don't consider yourself a
total beginner, you are better off reading the [Copper] book which approaches
the Rust on microcontrollers topic in a bottom-up and device agnostic way. Or,
maybe even just read the source of the [f3] crate.

[Copper]: https://japaric.github.io/copper
[f3]: https://github.com/japaric/f3
