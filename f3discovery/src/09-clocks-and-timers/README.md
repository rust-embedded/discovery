# Clocks and timers

In this section, we'll re-implement the LED roulette application. I'm going to give you back the
`Led` abstraction but this time I'm going to take away the `Delay` abstraction `:-)`.

Here's the starter code. The `delay` function is unimplemented so if you run this program the LEDs
will blink so fast that they'll appear to always be on.

``` rust
{{#include src/main.rs}}
```
