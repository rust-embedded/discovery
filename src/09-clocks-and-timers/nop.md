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
$ cargo objdump --bin clocks-and-timers --release -- -d -no-show-raw-insn
clocks-and-timers:      file format ELF32-arm-little

Disassembly of section .text:
clocks_and_timers::delay::h711ce9bd68a6328f:
 8000188:       push    {r4, r5, r7, lr}
 800018a:       movs    r4, #0
 800018c:       adds    r4, #1
 800018e:       uxth    r5, r4
 8000190:       bl      #4666
 8000194:       cmp     r5, #150
 8000196:       blo     #-14 <clocks_and_timers::delay::h711ce9bd68a6328f+0x4>
 8000198:       pop     {r4, r5, r7, pc}
```

Now, test this: Compile the program in debug mode and run it, then compile the program in release
mode and run it. What's the difference between them? What do you think is the main cause of the
difference? Can you think of a way to make them equivalent or at least more similar again?
