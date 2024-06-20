# It blinks

## Delaying
Now we're going to take a brief look into delay abstractions provided by `embedded-hal`
before combining this with the GPIO abstractions from the previous chapter in order to
finally make an LED blink.

`embedded-hal` provides us with an abstractions to delay the execution of our program:
[`DelayNs`]. This abstraction provides three functions `delay_ns`, `delay_us` and `delay_ms`
that delays execution for nano, micro or mili seconds respectively. They essentially work
the exact same way except that they accept different units for their delay function.

[`DelayNs`]: https://docs.rs/embedded-hal/1.0.0/embedded_hal/blocking/delay/trait.DelayNs.html

Inside our MCU, several so-called "timers" exist. They can do various things regarding time for us,
including simply pausing the execution of our program for a fixed amount of time. A very
simple delay-based program that prints something every second might for example look like this:

``` rust
{{#include examples/it-blinks-1.rs}}
```

Note that we changed our panic implementation from `panic_halt` to
`panic_rtt_target` here. This will require you to uncomment the two
RTT lines from `Cargo.toml` and comment the `panic-halt` one out,
since Rust only allows one panic implementation at a time.

In order to actually see the prints we have to change `Embed.toml` like shown on the marked lines (`<--- Here`):
```toml
[default.general]
# chip = "nrf52833_xxAA" # uncomment this line for micro:bit V2
# chip = "nrf51822_xxAA" # uncomment this line for micro:bit V1

[default.reset]
halt_afterwards = false     <--- Here

[default.rtt]
enabled = true              <--- Here

[default.gdb]
enabled = false             <--- Here
```

And now after putting the code into `src/main.rs` and another quick `cargo embed` (again with the same flags you used before)
you should see "`1000 ms passed`" being sent to your console every second from your MCU.

## Blinking

Now we've arrived at the point where we can combine our new knowledge about GPIO and delay abstractions
in order to actually make an LED on the back of the micro:bit blink. The resulting program is really just
a mash-up of the one above and the one that turned an LED on in the last section and looks like this:

``` rust
{{#include examples/it-blinks-2.rs}}
```

And after putting the code into `src/main.rs` and a final `cargo embed` (with the proper flags)
you should see the LED we light up before blinking as well as a print, every time the LED changes from off to on and vice versa.
