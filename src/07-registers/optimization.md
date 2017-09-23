# (mis)Optimization

Reads/writes to registers are quite special. I may even dare to say that they
are embodiment of side effects. In the previous example we wrote four different
values to the same register. If you didn't know that address was a register, you
may have simplified the logic to just write the final value `1 << (11 + 16)`
into the register.

Actually, LLVM, the compiler, does not know this is a register and will merge
the writes thus changing the behavior of our program. Let's check that really
quick.

```
$ xargo build --target thumbv7em-none-eabihf --release

$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/registers

(gdb) disassemble /m
Dump of assembler code for function main:
   0x080001da <+0>:     movw    r0, #4120       ; 0x1018
   0x080001de <+4>:     mov.w   r1, #134217728  ; 0x8000000
=> 0x080001e2 <+8>:     movt    r0, #18432      ; 0x4800
   0x080001e6 <+12>:    str     r1, [r0, #0]
   0x080001e8 <+14>:    b.n     0x80001e8 <main+14>
End of assembler dump.

(gdb) stepi
0x080001e6 in main ()

(gdb) stepi
0x080001e8 in main ()
```

The state of the LEDs didn't change this time! The `str` instruction is the one
that writes a value to the register. Our "debug" program had four of them, one
for each write to the register, but the "release" program only has one.

We can check that using `objdump`:

```
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/debug/registers
0800021c <main>:
 800021c:       b082            sub     sp, #8
 800021e:       e7ff            b.n     8000220 <main+0x4>
 8000220:       e7ff            b.n     8000222 <main+0x6>
 8000222:       f241 0018       movw    r0, #4120       ; 0x1018
 8000226:       f6c4 0000       movt    r0, #18432      ; 0x4800
 800022a:       f44f 7100       mov.w   r1, #512        ; 0x200
 800022e:       6001            str     r1, [r0, #0] <--
 8000230:       e7ff            b.n     8000232 <main+0x16>
 8000232:       f241 0018       movw    r0, #4120       ; 0x1018
 8000236:       f6c4 0000       movt    r0, #18432      ; 0x4800
 800023a:       f44f 6100       mov.w   r1, #2048       ; 0x800
 800023e:       6001            str     r1, [r0, #0] <--
 8000240:       e7ff            b.n     8000242 <main+0x26>
 8000242:       e7ff            b.n     8000244 <main+0x28>
 8000244:       f241 0018       movw    r0, #4120       ; 0x1018
 8000248:       f6c4 0000       movt    r0, #18432      ; 0x4800
 800024c:       f04f 7100       mov.w   r1, #33554432   ; 0x2000000
 8000250:       6001            str     r1, [r0, #0] <--
 8000252:       e7ff            b.n     8000254 <main+0x38>
 8000254:       e7ff            b.n     8000256 <main+0x3a>
 8000256:       f241 0018       movw    r0, #4120       ; 0x1018
 800025a:       f6c4 0000       movt    r0, #18432      ; 0x4800
 800025e:       f04f 6100       mov.w   r1, #134217728  ; 0x8000000
 8000262:       6001            str     r1, [r0, #0] <--
 8000264:       e7ff            b.n     8000266 <main+0x4a>
 8000266:       e7fe            b.n     8000266 <main+0x4a>
```

How do we prevent LLVM from misoptimizing our program? We use *volatile*
operations instead of plain reads/writes:

``` rust
pub fn main() -> ! {
    use core::ptr;

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

    loop {}
}
```

If we look at the disassemble of this new program compiled in release mode:

```
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/release/registers

080001da <main>:
 80001da:       f241 0018       movw    r0, #4120       ; 0x1018
 80001de:       f44f 7100       mov.w   r1, #512        ; 0x200
 80001e2:       f6c4 0000       movt    r0, #18432      ; 0x4800
 80001e6:       6001            str     r1, [r0, #0] <--
 80001e8:       f44f 6100       mov.w   r1, #2048       ; 0x800
 80001ec:       6001            str     r1, [r0, #0] <--
 80001ee:       f04f 7100       mov.w   r1, #33554432   ; 0x2000000
 80001f2:       6001            str     r1, [r0, #0] <--
 80001f4:       f04f 6100       mov.w   r1, #134217728  ; 0x8000000
 80001f8:       6001            str     r1, [r0, #0] <--
 80001fa:       e7fe            b.n     80001fa <main+0x20>
```

We see that the four writes (`str` instructions) are preserved. If you run it,
you'll also see that behavior of the program is preserved.
