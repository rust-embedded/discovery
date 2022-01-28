# Hardware/knowledge requirements

The primary knowledge requirement to read this book is to know *some* Rust. It's
hard for me to quantify *some* but at least I can tell you that you don't need
to fully grok generics, but you do need to know how to *use* closures. You also
need to be familiar with the idioms of the [2018 edition], in particular with
the fact that `extern crate` is not necessary in the 2018 edition.

[2018 edition]: https://rust-lang-nursery.github.io/edition-guide/

Also, to follow this material you'll need the following hardware:

- A [micro:bit v2] board, alternatively a [micro:bit v1.5] board, the book
  will refer to the v1.5 as just v1.

[micro:bit v2]: https://tech.microbit.org/hardware/
[micro:bit v1.5]: https://tech.microbit.org/hardware/1-5-revision/

(You can purchase this board from several [electronics][0] [suppliers][1])

[0]: https://microbit.org/buy/
[1]: https://www.mouser.com/microbit/_/N-aez3t?P=1y8um0l

<p align="center">
<img title="micro:bit" src="../assets/microbit-v2.jpg">
</p>

> **NOTE** This is an image of a micro:bit v2, the front of the v1 looks slightly different

- One micro-B USB cable, required to make the micro:bit board work.
  Make sure that the cable supports data transfer as some cables only support charging devices.

<p align="center">
<img title="micro-B USB cable" src="../assets/usb-cable.jpg">
</p>

> **NOTE** You may already have a cable like this, as some micro:bit kits ship with such cables.
> Some USB cables used to charge mobile devices may also work, if they are micro-B and have the
> capability to transmit data.

> **FAQ**: Wait, why do I need this specific hardware?

It makes my life and yours much easier.

The material is much, much more approachable if we don't have to worry about hardware differences.
Trust me on this one.

> **FAQ**: Can I follow this material with a different development board?

Maybe? It depends mainly on two things: your previous experience with microcontrollers and/or
whether a high level crate already exists, like the [`nrf52-hal`], for your development board
somewhere. You can look through the [Awesome Embedded Rust HAL list] for your microcontroller,
if you intend to use a different one.

[`nrf52-hal`]: https://docs.rs/nrf52-hal
[Awesome Embedded Rust HAL list]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates

With a different development board, this text would lose most if not all its beginner friendliness
and "easy to follow"-ness, IMO.

If you have a different development board and you don't consider yourself a total beginner, you are
better off starting with the [quickstart] project template.

[quickstart]: https://rust-embedded.github.io/cortex-m-quickstart/cortex_m_quickstart/
