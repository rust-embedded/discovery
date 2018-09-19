# Reverse a string

Alright, next let's make the server more interesting by having it respond to the client with the
reverse of the text that they sent. The server will respond to the client every time they press the
ENTER key. Each server response will be in a new line.

This time you'll need a buffer; you can use [`heapless::Vec`]. Here's the starter code:

[`heapless::Vec`]: https://docs.rs/heapless/0.2.1/heapless/struct.Vec.html

``` rust
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

        // TODO Receive a user request. Each user request ends with ENTER
        // NOTE `buffer.push` returns a `Result`. Handle the error by responding
        // with an error message.

        // TODO Send back the reversed string
    }
}
```
