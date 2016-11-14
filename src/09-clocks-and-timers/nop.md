# NOP

If in the previous section you compiled the program in release mode and actually
looked at the disassembly, you probably noticed that the `delay` function got
optimized away and never got called from within `main`.

LLVM decided that the function wasn't doing anything worthwhile and just removed
it.

There is a way to prevent LLVM from optimizing the `for` loop delay: a volatile
assembly instruction. Any instruction will do but NOP (No OPeration) is a
particular good choice in this case because it actually does nothing.

Your `for` loop delay would become:

``` rust
#[inline(never)]
fn delay(ms: u16) {
    for _ in 1_000 {
        unsafe { asm!("nop" :::: "volatile") }
    }
}
```

And this time it won't be compiled away by LLVM when you compile your program in
release mode:

```
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/release/clocks-and-timers

080001da <clocks_and_timers::delay::hc83787721a209f96>:
 80001da:       f44f 707a       mov.w   r0, #1000       ; 0x3e8
 80001de:       3801            subs    r0, #1
 80001e0:       bf00            nop
 80001e2:       d1fc            bne.n   80001de <clocks_and_timers::delay::hc83787721a209f96+0x4>
 80001e4:       4770            bx      lr
```

Now, test this: Compile the program in debug mode and run it then compile the
program in release mode and run it. What's the difference between them? What do
you think is the main cause of the difference? Can you think of a way to make
them equivalent or at least more similar again?
