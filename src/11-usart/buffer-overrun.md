# Overruns

If you wrote your program like this:

``` rust
#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let usart1 = unsafe { peripheral::usart1_mut() };

    // Send a string
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        usart1.tdr.write(|w| w.tdr(u16::from(*byte)));
    }

    unsafe { bkpt!() }

    loop {}
}
```

You probably received something like this on your laptop when you executed the
program compiled in debug mode.

```
# minicom's terminal
The uik ron fx ums oerth lzy og
```

And if you compiled in release mode, you probably only got something like this:

```
# minicom's terminal
Tq
```

What went wrong?

You see, sending bytes over the wire takes a relatively large amount of time. I
already did the math so let me quote myself:

> With a common configuration of 1 start bit, 8 bits of payload, zero stop bits
> and a baud rate of 115200 bps one can, in theory, send 12,800 frames of 9 bits
> per second. Since each one carries a byte of information that results in a
> data rate of 12.8 KB/s.

Our pangram has a length of 45 bytes. That means it's going to take, at least,
3,500 microseconds (`45 bytes / (12,800 bytes/s) = 3,515 us`) to send the
string. The processor is working at 8 MHz, where executing an instruction takes
125 nanoseconds, so it's likely going to be done with the `for` loop is less
than 3,500 microseconds.

We can actually time how long it takes to execute the `for` loop. There's a
`time` module in the `pg` crate that exposes an `Instant` API that's similar to
the one in `std::time`.

``` rust
#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    use pg::time::{FREQUENCY, Instant};

    let usart1 = unsafe { peripheral::usart1_mut() };

    let instant = Instant::now();
    // Send a string
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        usart1.tdr.write(|w| w.tdr(u16::from(*byte)));
    }
    let elapsed = instant.elapsed(); // ticks

    iprintln!("`for` loop took {} ticks ({} us)",
              elapsed,
              elapsed as f32 / FREQUENCY as f32 * 1e6);

    unsafe { bkpt!() }

    loop {}
}
```

In debug mode, I get:

```
# itmdump's terminal
`for` loop took 21614 ticks (2701.75 us)
```

This is less than 3,500 microseconds but it's not that far off and that's why
only a few bytes of information are lost.

In conclusion, the processor is trying to send bytes at a faster rate than what
the hardware can actually handle and this results in data loss. This condition
is known as buffer *overrun*.

How do we avoid this? The status register (`ISR`) has a flag, `TXE`, that
indicates if it's "safe" to write to the `TDR` register without incurring in
data loss.

Let's use that to slowdown the processor.

``` rust
#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let usart1 = unsafe { peripheral::usart1_mut() };

    let instant = Instant::now();
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        // wait until it's safe to write to TDR
        while !usart1.isr.read().txe() {}

        usart1.tdr.write(|w| w.tdr(u16::from(*byte)));
    }
    let elapsed = instant.elapsed(); // ticks

    iprintln!("`for` loop took {} ticks ({} us)",
              elapsed,
              elapsed as f32 / FREQUENCY as f32 * 1e6);

    unsafe { bkpt!() }

    loop {}
}
```

This time, running the program in debug or release mode should result in a
complete string on the receiving side.

```
# minicom/PuTTY's console
The quick brown fox jumps over the lazy dog.
```

The timing of the `for` loop should be closer to the theoretical 3,500
microseconds as well. The timing below is for the "debug" version.

```
# itmdump's console
`for` loop took 30397 ticks (3799.625 us)
```
