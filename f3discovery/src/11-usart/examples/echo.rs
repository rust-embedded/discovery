<<<<<<< HEAD:src/11-usart/my-solution.md
<!-- # My solution -->

# 解答例

```rust
#![deny(unsafe_code)]
=======
>>>>>>> upstream/master:f3discovery/src/11-usart/examples/echo.rs
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::Vec;

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

<<<<<<< HEAD:src/11-usart/my-solution.md
    // 32バイト容量のバッファ
    let mut buffer: Vec<u8, consts::U32> = Vec::new();
=======
    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();
>>>>>>> upstream/master:f3discovery/src/11-usart/examples/echo.rs

    loop {
        buffer.clear();

        loop {
            while usart1.isr.read().rxne().bit_is_clear() {}
            let byte = usart1.rdr.read().rdr().bits() as u8;

            if buffer.push(byte).is_err() {
                // バッファが満杯
                for byte in b"error: buffer full\n\r" {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1
                        .tdr
                        .write(|w| w.tdr().bits(u16::from(*byte)));
                }

                break;
            }

            // キャリッジリターン
            if byte == 13 {
                // 返信
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1
                        .tdr
                        .write(|w| w.tdr().bits(u16::from(*byte)));
                }

                break;
            }
        }
    }
}
