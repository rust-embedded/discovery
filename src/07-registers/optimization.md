# (mis)Optimization

Reads/writes to registers are quite special. I may even dare to say that they are embodiment of side
effects. In the previous example we wrote four different values to the same register. If you didn't
know that address was a register, you may have simplified the logic to just write the final value `1
<< (11 + 16)` into the register.

Actually, LLVM, the compiler's backend / optimizer, does not know we are dealing with a register and
will merge the writes thus changing the behavior of our program. Let's check that really quick.

``` console
$ cargo run --release

(gdb) next

(gdb) disassemble /m
Dump of assembler code for function cortex_m_rt::reset_handler::main:
   0x08000672 <+0>:     push    {r7, lr}
   0x08000674 <+2>:     mov     r7, sp
   0x08000676 <+4>:     movw    r0, #392        ; 0x188
   0x0800067a <+8>:     movt    r0, #2048       ; 0x800
   0x0800067e <+12>:    ldrb    r0, [r0, #0]
   0x08000680 <+14>     bl      8000548 <aux7::init>
   0x08000684 <+18>     movw    r0, #4120       ; 0x1018
   0x08000688 <+22>     mov.w   r1, #134217728  ; 0x8000000
   0x0800068c <+26>     movt    r0, #18432      ; 0x4800
   0x08000690 <+30>     str     r1, [r0, #0]    ; <- 1
=> 0x08000692 <+32>:    pop     {r7, pc}

End of assembler dump.
```

The state of the LEDs didn't change this time! The `str` instruction is the one that writes a value
to the register. Our *debug* (unoptimized) program had four of them, one for each write to the
register, but the *release* (optimized) program only has one.

We can check that using `objdump`:

``` console
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/debug/registers
080001ec <registers::main>:
 80001ec:       b580            push    {r7, lr}
 80001ee:       466f            mov     r7, sp
 80001f0:       b082            sub     sp, #8
 80001f2:       f000 f85a       bl      80002aa <aux7::init>
 80001f6:       9001            str     r0, [sp, #4]
 80001f8:       e7ff            b.n     80001fa <registers::main+0xe>
 80001fa:       e7ff            b.n     80001fc <registers::main+0x10>
 80001fc:       f241 0018       movw    r0, #4120       ; 0x1018
 8000200:       f6c4 0000       movt    r0, #18432      ; 0x4800
 8000204:       f44f 7100       mov.w   r1, #512        ; 0x200
 8000208:       6001            str     r1, [r0, #0]    ; <- 1
 800020a:       e7ff            b.n     800020c <registers::main+0x20>
 800020c:       f241 0018       movw    r0, #4120       ; 0x1018
 8000210:       f6c4 0000       movt    r0, #18432      ; 0x4800
 8000214:       f44f 6100       mov.w   r1, #2048       ; 0x800
 8000218:       6001            str     r1, [r0, #0]    ; <- 2
 800021a:       e7ff            b.n     800021c <registers::main+0x30>
 800021c:       e7ff            b.n     800021e <registers::main+0x32>
 800021e:       f241 0018       movw    r0, #4120       ; 0x1018
 8000222:       f6c4 0000       movt    r0, #18432      ; 0x4800
 8000226:       f04f 7100       mov.w   r1, #33554432   ; 0x2000000
 800022a:       6001            str     r1, [r0, #0]    ; <- 3
 800022c:       e7ff            b.n     800022e <registers::main+0x42>
 800022e:       e7ff            b.n     8000230 <registers::main+0x44>
 8000230:       f241 0018       movw    r0, #4120       ; 0x1018
 8000234:       f6c4 0000       movt    r0, #18432      ; 0x4800
 8000238:       f04f 6100       mov.w   r1, #134217728  ; 0x8000000
 800023c:       6001            str     r1, [r0, #0]    ; <- 4
 800023e:       b002            add     sp, #8
 8000240:       bd80            pop     {r7, pc}
```

How do we prevent LLVM from misoptimizing our program? We use *volatile* operations instead of plain
reads/writes:

``` rust
fn main() {
    use core::ptr;

    aux7::init();

    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
    }
}
```

If we look at the disassemble of this new program compiled in release mode:

``` console
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/release/registers
08000672 <cortex_m_rt::reset_handler::main>:
 8000672:       b580            push    {r7, lr}
 8000674:       466f            mov     r7, sp
 8000676:       f240 1088       movw    r0, #392        ; 0x188
 800067a:       f6c0 0000       movt    r0, #2048       ; 0x800
 800067e:       7800            ldrb    r0, [r0, #0]
 8000680:       f7ff ff62       bl      8000548 <aux7::init>
 8000684:       f241 0018       movw    r0, #4120       ; 0x1018
 8000688:       f44f 7100       mov.w   r1, #512        ; 0x200
 800068c:       f6c4 0000       movt    r0, #18432      ; 0x4800
 8000690:       6001            str     r1, [r0, #0]    ; <- 1
 8000692:       f44f 6100       mov.w   r1, #2048       ; 0x800
 8000696:       6001            str     r1, [r0, #0]    ; <- 2
 8000698:       f04f 7100       mov.w   r1, #33554432   ; 0x2000000
 800069c:       6001            str     r1, [r0, #0]    ; <- 3
 800069e:       f04f 6100       mov.w   r1, #134217728  ; 0x8000000
 80006a2:       6001            str     r1, [r0, #0]    ; <- 4
 80006a4:       bd80            pop     {r7, pc}
```

We see that the four writes (`str` instructions) are preserved. If you run it (use `stepi`), you'll
also see that behavior of the program is preserved.
