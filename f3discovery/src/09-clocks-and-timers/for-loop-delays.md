# `for` loop delays

The first challenge is to implement the `delay` function without using any peripheral and the
obvious solution is to implement it as a `for` loop delay:

``` rust
#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    for _ in 0..1_000 {}
}
```

Of course, the above implementation is wrong because it always generates the same delay for any
value of `ms`.

In this section, you'll have to:

- Fix the `delay` function to generate delays proportional to its input `ms`.
- Tweak the `delay` function to make the LED roulette spin at a rate of approximately 5 cycles in 4
  seconds (800 milliseconds period).
- The processor inside the microcontroller is clocked at 72 MHz and executes most instructions in one
  "tick", a cycle of its clock. How many (`for`) loops do  you *think* the `delay` function must do
  to generate a delay of 1 second?
- How many `for` loops does `delay(1000)` actually do?
- What happens if compile your program in release mode and run it?
