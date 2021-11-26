# UART

The microcontroller has a peripheral called UART, which stands for Universal
Asynchronous Receiver/Transmitter. This peripheral can be configured to work with
several communication protocols like the serial communication protocol.


Throughout this chapter, we'll use serial communication to exchange information between the
microcontroller and your computer.

> **NOTE** that on the micro:bit v2 we will use the so called UARTE peripheral which behaves
> just like a regular UART, except that the HAL has to talk to it differently.
> However, this will of course not be our concern.

## Setup
As always from now on you will have to modify the `Embed.toml` to match your micro:bit version:

```toml
{{#include Embed.toml}}
```
