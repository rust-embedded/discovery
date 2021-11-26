# Send a single byte

Our first task will be to send a single byte from the microcontroller to the computer over the serial
connection.

In order to do that we will use the following snippet (this one is already in `07-uart/src/main.rs`):

``` rust
{{#include src/main.rs}}
```

The most prevalent new thing here is obviously the `cfg` directives to conditionally include/exclude
parts of the code. This is mostly just because we want to work with a regular UART for the micro:bit v1
and with the UARTE for micro:bit v2.

You will also have noticed that this is the first time we are including some code that is not from a library,
namely the `serial_setup` module. Its only purpose is to provide a nice wrapper around the UARTE
so we can use it the exact same way as the UART via the [`embedded_hal::serial`] traits. If you want, you can
check out what exactly the module does, but it is not required to understand this chapter in general.

[`embedded_hal::serial`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/serial/index.html

Apart from those differences, the initialization procedures for the UART and the UARTE are quite similar so we'll
discuss the initialization of just UARTE. The UARTE is initialized with this piece of code:
```rs
uarte::Uarte::new(
    board.UARTE0,
    board.uart.into(),
    Parity::EXCLUDED,
    Baudrate::BAUD115200,
);
```
This function takes ownership of the UARTE peripheral representation in Rust (`board.UARTE0`) and the TX/RX pins
on the board (`board.uart.into()`) so nobody else can mess with either the UARTE peripheral or our pins while
we are using them. After that we pass two configuration options to the constructor: the baudrate (that one should be
familiar) as well as an option called "parity". Parity is a way to allow serial communication lines to check whether
the data they received was corrupted during transmission. We don't want to use that here so we simply exclude it.
Then we wrap it up in the `UartePort` type so we can use it the same way as the micro:bit v1's `serial`.

After the initialization, we send our `X` via the newly created uart instance. The `block!` macro here is the `nb::block!`
macro. `nb` is a (quoting from its description) "Minimal and reusable non-blocking I/O layer". It allows us to write
code that can conduct hardware operations in the background while we go and do other work (non-blocking). However,
in this and many other cases we have no interest in doing some other work so we just call `block!` which will wait until
the I/O operation is done and has either succeeded or failed and then continue execution normally.

Last but not least, we `flush()` the serial port. This is because an implementor of the `embedded-hal::serial` traits may
decide to buffer output until it has received a certain number of bytes to send (this is the case with the UARTE implementation).
Calling `flush()` forces it to write the bytes it currently has right now instead of waiting for more.

## Testing it

Before flashing this you should make sure to start your minicom/PuTTY as the data we receive via our serial
communication is not backed up or anything, we have to view it live. Once your serial monitor is up you can
flash the program just like in chapter 5:
```
# For micro:bit v2
$ cargo embed --features v2 --target thumbv7em-none-eabihf
  (...)

# For micro:bit v1
$ cargo embed --features v1 --target thumbv6m-none-eabi
```

And after the flashing is finished, you should see the character `X` show up on your minicom/PuTTY terminal, congrats!
