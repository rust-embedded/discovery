# NOP

If in the previous section you compiled the program in release mode and actually looked at the
disassembly, you probably noticed that the `delay` function is optimized away and never gets called
from within `main`.

LLVM decided that the function wasn't doing anything worthwhile and just removed it.

There is a way to prevent LLVM from optimizing the `for` loop delay: add a *volatile* assembly
instruction. Any instruction will do but NOP (No OPeration) is a particular good choice in this case
because it has no side effect.

Your `for` loop delay would become:

``` rust
#[inline(never)]
fn delay(_tim6: &tim6::RegisterBlock, ms: u16) {
    const K: u16 = 3; // this value needs to be tweaked
    for _ in 0..(K * ms) {
        aux9::nop()
    }
}
```

And this time `delay` won't be compiled away by LLVM when you compile your program in release mode:

``` console
$ cargo build --release
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/release/clocks-and-timers
08000548 <clocks_and_timers::delay>:
 8000548:       2000            movs    r0, #0
 800054a:       3001            adds    r0, #1
 800054c:       bf00            nop
 800054e:       b281            uxth    r1, r0
 8000550:       2996            cmp     r1, #150        ; 0x96
 8000552:       d3fa            bcc.n   800054a <clocks_and_timers::delay+0x2>
 8000554:       4770            bx      lr
```

Now, test this: Compile the program in debug mode and run it, then compile the program in release
mode and run it. What's the difference between them? What do you think is the main cause of the
difference? Can you think of a way to make them equivalent or at least more similar again?
