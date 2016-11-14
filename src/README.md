# Discovery

> Discover the world of microcontrollers through [Rust]

[Rust]: https://www.rust-lang.org/en-US/

This book is an introductory course in microcontroller-based "embedded systems"
using Rust as the teaching language rather than the usual C/C++.

## Approach

- Beginner friendly. No previous experience with microcontrollers or embedded
  systems is required.

- Hands on. Plenty of exercises to put the theory into practice. *You* will be
  doing most of the work here.

- Tool centered. We'll make plenty use of tooling to ease development. Debugging
  and logging will be introduced early on. Using LEDs as a debugging mechanism
  has no place here.

## Scope

The following topics will be covered (eventually, I hope):

- Functionality ("peripherals") commonly found in microcontrollers: Digital
  input and output, Pulse Width Modulation (PWM), Analog to Digital Converters
  (ADC), and common communication protocols: Serial, I2C, SPI, etc.

- Multitasking concepts: cooperative vs preemptive multitasking, interrupts,
  schedulers, etc.

- Control systems concepts: sensors, calibration, digital filters, actuators,
  open loop control, closed loop control, etc.

## Non-goals

- Teaching Rust. There's plenty of material on that topic already. We'll focus
  on microcontrollers and embedded systems.

- Being a comprehensive text about electric circuit theory or electronics.
  We'll just cover the minimum required to understand how some devices work.

- Port this material to any other development board. I only have so much time.
  This book will make exclusive use of the STM32F3DISCOVERY board.

- Cover Rustic, low level details. We won't be talking about linker scripts,
  the boot process or how to glue those two into a minimally working Rust
  program. The [Copper] book has information on those topics though.

[Copper]: https://japaric.github.io/copper/
