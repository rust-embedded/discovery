# My solution

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::{consts, Vec};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            while usart1.isr.read().rxne().bit_is_clear() {}
            let byte = usart1.rdr.read().rdr().bits() as u8;

            if buffer.push(byte).is_err() {
                // buffer full
                for byte in b"error: buffer full\n\r" {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }

                break;
            }

            // Carriage return
            if byte == 13 {
                // Respond
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }

                break;
            }
        }
    }
}
```
