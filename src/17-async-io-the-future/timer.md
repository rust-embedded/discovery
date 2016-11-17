# Timer

The first API we'll rediscover is `Timer` which deprecates the `delay` module.

Here's an example of this API.

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use core::iter;

use pg::led::LEDS;
use pg::{Async, Future, Timer};

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let mut timer = Timer::new().unwrap();

    let mut periodic = timer.periodic(100);
    let mut leds = LEDS.iter()
        .zip(LEDS.iter().skip(1))
        .chain(iter::once((&LEDS[7], &LEDS[0])))
        .cycle();
    loop {
        if let Async::Ready(()) = periodic.poll() {
            if let Some((current, next)) = leds.next() {
                current.off();
                next.on();
            }
        }
    }
}
```

The first thing you notice is that we need to create an instance of `Timer` to
be able to generate delays. `Timer::new` will create a unique instance of the
timer and thus will return `Some` only the first time is called; subsequent
calls of this constructor will return `None`. We'll return to this requirement
of uniqueness later on.

`Timer` provides a `periodic` method that returns an implementer of the `Future`
trait: `Periodic`. Polling `Periodic` will return `Async::Ready` only after the
requested timeout period, 500 milliseconds in this case, has elapsed.

`Periodic` also happens to be an "infinite stream" because it can yield an
infinite number of `Async::Ready` values.

`Timer` provides another method, `oneshot`. Which can be used to emulate the old
`delay` module. In fact, `timer.oneshot(100).wait()` is equivalent to the
`delay::ms` function because it also uses busy waiting to block for 100
milliseconds.

The `oneshot` method actually returns a `Future` implementer, `OneShot`, which
can be used in an asynchronous manner as well. This actually means that this
asynchronous API is a super set of the synchronous one because the synchronous
behavior can be easily achieved using the `wait` method.

Back to the issue of uniqueness. `Timer` uses the `TIM7` peripheral under the
hood. If we allowed creation of multiple instances of it, that would make code
like this compile:

```
let mut timer1 = Timer::new();
let mut timer2 = Timer::new();

let delay1 = timer.oneshot(100);
let delay2 = timer.oneshot(200);

delay1.wait();  // this actually blocks for 200 milliseconds!
```

But this won't work as expected because the second `oneshot` call will
"overwrite" the previous `oneshot` call as both of these methods would end
up writing to the same registers.
