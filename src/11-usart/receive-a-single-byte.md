# Receive a single byte

So far we have sending data from the micro to your laptop. It's time to try the opposite: receiving
data from your laptop.

There's a `RDR` register that will be filled with the data that comes from the RX line. If we read
that register, we'll retrieve the data that the other side of the channel sent. The question is: How
do we know that we have received (new) data? The status register, `ISR`, has a bit for that purpose:
`RXNE`. We can just busy wait on that flag.

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
minicom/PuTTY's console. What happens? What are the contents of the `_byte` variable?

```
(gdb) continue
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
__bkpt () at asm/bkpt.s:3

(gdb) finish
Run till exit from #0  __bkpt () at asm/bkpt.s:3
usart::main () at src/11-usart/src/main.rs:23
23              aux11::bkpt();

(gdb) p/c _byte
$1 = 97 'a'
```
