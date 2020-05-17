# Send a single byte

Our first task will be to send a single byte from the microcontroller to the computer over the serial
connection.

This time, I'm going to provide you with an already initialized USART peripheral. You'll only have
to work with the registers that are in charge of sending and receiving data.

Go into the `11-usart` directory and let's run the starter code therein. Make sure that you have
minicom/PuTTY open.

``` rust
{{#include src/main.rs}}
```

This program writes to the `TDR` register. This causes the `USART` peripheral to send one byte of
information through the serial interface.

On the receiving end, your computer, you should see show the character `X` appear on minicom/PuTTY's
terminal.
