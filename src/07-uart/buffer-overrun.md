# Overruns

If you wrote your program like this:

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // Send a string
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
    }

    loop {}
}
```

You probably received something like this on your computer when you executed the program compiled in
debug mode.

``` console
$ # minicom's terminal
(..)
The uic brwn oxjums oer helaz do.
```

And if you compiled in release mode, you probably only got something like this:

``` console
$ # minicom's terminal
(..)
T
```

What went wrong?

You see, sending bytes over the wire takes a relatively large amount of time. I already did the math
so let me quote myself:

> With a common configuration of 1 start bit, 8 bits of data, 1 stop bit and a baud rate of 115200
> bps one can, in theory, send 11,520 frames per second. Since each one frame carries a byte of data
> that results in a data rate of 11.52 KB/s

Our pangram has a length of 45 bytes. That means it's going to take, at least, 3,900 microseconds
(`45 bytes / (11,520 bytes/s) = 3,906 us`) to send the string. The processor is working at 8 MHz,
where executing an instruction takes 125 nanoseconds, so it's likely going to be done with the `for`
loop in less than 3,900 microseconds.

We can actually time how long it takes to execute the `for` loop. `aux11::init()` returns a
`MonoTimer` (monotonic timer) value that exposes an `Instant` API that's similar to the one in
`std::time`.

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    let instant = mono_timer.now();
    // Send a string
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
    }
    let elapsed = instant.elapsed(); // in ticks

    iprintln!(
        &mut itm.stim[0],
        "`for` loop took {} ticks ({} us)",
        elapsed,
        elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );

    loop {}
}
```

In debug mode, I get:

``` console
$ # itmdump terminal
(..)
`for` loop took 22415 ticks (2801.875 us)
```

This is less than 3,900 microseconds but it's not that far off and that's why only a few bytes of
information are lost.

In conclusion, the processor is trying to send bytes at a faster rate than what the hardware can
actually handle and this results in data loss. This condition is known as buffer *overrun*.

How do we avoid this? The status register (`ISR`) has a flag, `TXE`, that indicates if it's "safe"
to write to the `TDR` register without incurring in data loss.

Let's use that to slowdown the processor.

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    let instant = mono_timer.now();
    // Send a string
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        // wait until it's safe to write to TDR
        while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!

        usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
    }
    let elapsed = instant.elapsed(); // in ticks

    iprintln!(
        &mut itm.stim[0],
        "`for` loop took {} ticks ({} us)",
        elapsed,
        elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );

    loop {}
}
```

This time, running the program in debug or release mode should result in a complete string on the
receiving side.

``` console
$ # minicom/PuTTY's console
(..)
The quick brown fox jumps over the lazy dog.
```

The timing of the `for` loop should be closer to the theoretical 3,900 microseconds as well. The
timing below is for the debug version.

``` console
$ # itmdump terminal
(..)
`for` loop took 30499 ticks (3812.375 us)
```
