# Reverse a string

Alright, next let's make the server more interesting by having it respond to the
client with the reverse of the text that they sent. The server will respond to
the client every time they press the ENTER key. Each server response will be
in a new line.

This time you'll need a buffer; you can use `FixedVec`. Here's the starter code:

``` rust
#[macro_use]
extern crate fixedvec;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    use fixedvec::{FixedVec, Result};

    let usart1 = unsafe { peripheral::usart1_mut() };

    let mut memory = alloc_stack!([u8; 32]);
    let mut buffer = FixedVec::new(&mut memory);
    loop {
        buffer.clear();

        // TODO Receive a user request. Each user request ends with ENTER
        // NOTE `buffer.push` returns a `Result`. Handle the error by responding
        // with an error message.

        // TODO Send back the reversed string
    }
}
```
