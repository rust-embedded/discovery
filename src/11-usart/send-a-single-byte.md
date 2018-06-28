# Send a single byte

Our first task will be to send a single byte from the microcontroller to the laptop over the serial
connection.

This time, I'm going to provide you with an already initialized USART peripheral. You'll only have
to work with the registers that are in charge of sending and receiving data.

Go into the `11-usart` directory and let's run the starter code therein. Make sure that you have
minicom/PuTTY open.

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate aux11;
#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;

entry!(main);

fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // Send a single character
    usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));

    loop {}
}
```

This program writes to the `TDR` register. This causes the `USART` peripheral to send one byte of
information through the serial interface.

On the receiving end, your laptop, you should see show the character `X` appear on minicom/PuTTY's
terminal.
