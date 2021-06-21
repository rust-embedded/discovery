# Receive a single byte

So far we have sending data from the microcontroller to your computer. It's time to try the opposite: receiving
data from your computer.

There's a `RDR` register that will be filled with the data that comes from the RX line. If we read
that register, we'll retrieve the data that the other side of the channel sent. The question is: How
do we know that we have received (new) data? The status register, `ISR`, has a bit for that purpose:
`RXNE`. We can just busy wait on that flag.

``` rust
{{#include examples/receive-a-single-byte.rs}}
```

Let's try this program! Let it run free using `continue` and then type a single character in
minicom/PuTTY's console. What happens? What are the contents of the `_byte` variable?

```
(gdb) continue
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
0x8003d48 in __bkpt ()

(gdb) finish
Run till exit from #0  0x8003d48 in __bkpt ()
usart::main () at src/11-usart/src/main.rs:19
19              aux11::bkpt();

(gdb) p/c _byte
$1 = 97 'a'
```
