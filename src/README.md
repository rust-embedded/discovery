# Discovery

> Discover the world of microcontrollers through [Rust]!

[Rust]: https://www.rust-lang.org/en-US/

This book is an "introductory course" on microcontroller-based "embedded
systems" that uses Rust as the teaching language rather than the usual C/C++.

## Scope

The following topics will be covered (eventually, I hope):

- How to write, build, flash and debug an "embedded" (Rust) program.

- Functionality ("peripherals") commonly found in microcontrollers: Digital
  input and output, Pulse Width Modulation (PWM), Analog to Digital Converters
  (ADC), common communication protocols like Serial, I2C and SPI, etc.

- Multitasking concepts: cooperative vs preemptive multitasking, interrupts,
  schedulers, etc.

- Control systems concepts: sensors, calibration, digital filters, actuators,
  open loop control, closed loop control, etc.

## Approach

- Beginner friendly. No previous experience with microcontrollers or embedded
  systems is required.

- Hands on. Plenty of exercises to put the theory into practice. *You* will be
  doing most of the work here.

- Tool centered. We'll make plenty use of tooling to ease development. "Real"
  debugging, GDB, and logging will be introduced early on. Using LEDs as a
  debugging mechanism has no place here.

## Non-goals

What's out of scope for this book:

- Teaching Rust. There's plenty of material on that topic already. We'll focus
  on microcontrollers and embedded systems.

- Being a comprehensive text about electric circuit theory or electronics.
  We'll just cover the minimum required to understand how some devices work.

- Covering Rustic, low level details. We won't be talking about linker scripts,
  the boot process or how to glue those two into a minimally working Rust
  program. The [Copper] book has information on those topics though.

Also I don't intent to port this material to other development boards; this book
will make exclusive use of the STM32F3DISCOVERY development board.

[Copper]: https://japaric.github.io/copper/

## Sponsored by

<p align="center">
<a href="http://integer32.com/">
<img style="width: 50%" title="integer 32" src="assets/integer32.svg">
</a>
</p>

Many thanks to [integer 32](http://integer32.com/) for sponsoring me to continue
working on this book! Please give them lots of work (they do Rust consulting!)
so they'll have no choice but to hire more Rustaceans <3.
