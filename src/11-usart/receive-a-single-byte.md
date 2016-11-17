# Receive a single byte

So far we have sending data from the micro to your laptop. It's time to try the
opposite: receiving data from your laptop.

There's a `RDR` register that will be filled with the data that comes from the
RX line. If we read that register, we'll retrieve the data that the other side
of the channel sent. The question is: How do we know that we have received (new)
data? The status register (`ISR`) has a bit for that purpose: `RXNE`. We can
just busy wait on that flag.

``` rust
#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let usart1 = unsafe { peripheral::usart1_mut() };

    loop {
        // Wait until there's data available
        while !usart1.isr.read().rxne() {}

        // Retrieve the data
        let _byte = usart1.rdr.read().rdr() as u8;

        unsafe { bkpt!() };
    }
}
```

Let's try this program! Let it run free using `continue` and then type a single
character in minicom/PuTTY's console. What happens? What are the contents of the
`_byte` variable? Try:

```
(gdb) p/c _byte
```
