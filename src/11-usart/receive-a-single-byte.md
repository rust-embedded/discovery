# Receive a single byte

So far we have sending data from the micro to your laptop. It's time to try the opposite: receiving
data from your laptop.

There's a `RDR` register that will be filled with the data that comes from the RX line. If we read
that register, we'll retrieve the data that the other side of the channel sent. The question is: How
do we know that we have received (new) data? The status register, `ISR`, has a bit for that purpose:
`RXNE`. We can just busy wait on that flag.

``` rust
#![no_std]

extern crate aux11;

fn main() {
    let (usart1, _mono_timer, _itm) = aux11::init();

    loop {
        // Wait until there's data available
        while usart1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        let _byte = usart1.rdr.read().rdr().bits() as u8;

        aux11::bkpt();
    }
}
```

Let's try this program! Let it run free using `continue` and then type a single character in
minicom/PuTTY's console. What happens? What are the contents of the `_byte` variable? Try:

```
(gdb) p/c _byte
$1 = 88 'X'
```
