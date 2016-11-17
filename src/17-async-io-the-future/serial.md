# Serial

The second API we'll rediscover is `Serial` which replaces our old code that
involved direct register manipulation and busy waiting.

Here's an example of this API.

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use pg::{Async, Future, Serial};

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let Serial { mut rx, mut tx } = Serial::new().unwrap();

    let mut bytes = rx.bytes();
    loop {
        if let Async::Ready(byte) = bytes.poll() {
            // immediately echo back the byte we just received
            tx.write(byte).wait();
        }
    }
}
```

`Serial` contains two fields: `Rx` and `Tx` which provide an asynchronous API
over the receiver and transmitter parts of the serial interface.

One of `Rx` methods is `bytes` which returns a `Bytes` struct that represents an
infinite stream of bytes that are read from the receiver pin/line. The next byte
can be requested without blocking using the `poll` method.

`Tx` provides a `write` method that *queues* a write onto the TX line. The write
is not performed immediately because there may be pending write. `wait` can be
used to force the write; this may involve waiting for a pending write to end.
