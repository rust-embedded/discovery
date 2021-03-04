# Send a single byte

Our first task will be to send a single byte from the microcontroller to the computer over the serial
connection.

In order to do that we will use the following snippet (this one is already in `07-uart/src/main.rs`):

``` rust
{{#include src/main.rs}}
```


There are some parts we have already seen before but also lots of new stuff so lets focus on those.
First things first, how do we know that we have to interact with `P0_24` and `P0_25` here? If you take a look
at the micro:bit schematics again, in page 5 you will notice that those two pins are also referred to as `TGT_RX`
and `TGT_TX`. Not only that, they are also connected to our debug probe (see page 2) so those are most likely the two
pins we are looking for.

Next up, we can observe a new pin configuration we haven't seen before `p0.p0_25.into_floating_input()`. This just
means that `P0_25` is now an input. What exactly floating means is none of our concern right now. But how do we know
we have to put our pins in this exact mode? If you look at the line below you can see that we construct an instance
of `Pins` which is later passed on to our UART peripheral constructor (`cts` and `rts` are for more advanced UART features
we do not use here, hence they are set to `None`) and the [type signature](https://docs.rs/nrf51-hal/0.11.0/nrf51_hal/uart/struct.Pins.html)
of `Pins` already tells us what to do.

Next, we construct our UART peripheral with `hal::Uart::new(p.UART0, pins, hal::uart::Parity::EXCLUDED, hal::uart::Baudrate::BAUD115200);`.
This function takes ownership of `p.UART0` and our `pins` so nobody else can mess with either the UART peripheral or our pins while
we are using them. After that we pass two configuration options two the constructor: the baudrate (that one should be
familiar) as well as an option called "parity". Parity is a way that allows serial communication lines to check whether
the data they received was corrupted during transmission or not but we don't want to use that here so we simply exclude it.

Last but not least, we send our `X` via the newly created uart instance. The `block!` macro here is the `nb::block!`
macro. `nb` is a (quoting from its description) "Minimal and reusable non-blocking I/O layer". It allows us to write
code that can conduct hardware operations in the background while we go and do other work (non-blocking). However,
in this and many other cases we have no interest in doing some other work so we just call `block!` which will wait until
the I/O operation is done and has either succeeded or failed and then continue execution normally.

## Testing it

Before flashing this you should make sure to start your minicom/PuTTY as the data we receive via our serial
communication is not backed up or anything, we have to view it live. Once your serial monitor is up you can
do a simple

```
$ cargo embed --release
```

And after the flashing is finished, you should see the character `X` show up on your minicom/PuTTY terminal, congrats!
