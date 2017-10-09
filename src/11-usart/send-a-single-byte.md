# Send a single byte

Our first task will be to send a single byte from the microcontroller to the
laptop over the serial connection.

This time, I'm going to provide you with an already initialized USART
peripheral. You'll only have to work with the registers that are in charge of
sending data back and forth.

Go into the `11-usart` directory and let's run the starter code therein. Make
sure that you have minicom/PuTTY open.

``` rust
#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let usart1 = unsafe { peripheral::usart1_mut() };

    // Send a single character
    usart1.tdr.write(|w| w.tdr(u16::from('X' as u8)));

    loop {}
}
```

This program writes to the `TDR` register. This causes the `USART` peripheral
to send one byte of information through the serial interface.

On the receiving end, your laptop, you should see show the character `X` appear
on minicom/PuTTY's terminal.
