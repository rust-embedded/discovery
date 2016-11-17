# My solution

```rust
#![no_main]
#![no_std]

#[macro_use]
extern crate fixedvec;

#[macro_use]
extern crate pg;

use fixedvec::{FixedVec, Result};
use pg::peripheral;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let usart1 = unsafe { peripheral::usart1_mut() };

    let mut memory = alloc_stack!([u8; 32]);
    let mut buffer = FixedVec::new(&mut memory);
    loop {
        // Receive
        buffer.clear();
        match (|| -> Result<()> {
            loop {
                while !usart1.isr.read().rxne() {}
                let byte = usart1.rdr.read().rdr() as u8;

                try!(buffer.push(byte));

                // Carriage return
                if byte == 13 {
                    break;
                }
            }

            Ok(())
        })() {
            Err(_) => {
                for byte in b"error: buffer overflow\n\r" {
                    while !usart1.isr.read().txe() {}
                    usart1.tdr.write(|w| w.tdr(u16::from(*byte)));
                }
            }
            Ok(_) => {
                // Respond
                for byte in buffer.iter()
                    .rev()
                    .chain(&['\n' as u8, '\r' as u8]) {
                    while !usart1.isr.read().txe() {}
                    usart1.tdr.write(|w| w.tdr(u16::from(*byte)));
                }
            }
        }
    }
}
```
