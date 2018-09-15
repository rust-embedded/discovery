# (mis)Optimization

Reads/writes to registers are quite special. I may even dare to say that they are embodiment of side
effects. In the previous example we wrote four different values to the same register. If you didn't
know that address was a register, you may have simplified the logic to just write the final value `1
<< (11 + 16)` into the register.

Actually, LLVM, the compiler's backend / optimizer, does not know we are dealing with a register and
will merge the writes thus changing the behavior of our program. Let's check that really quick.

``` console
$ cargo run --release
(..)
Breakpoint 1, main () at src/07-registers/src/main.rs:9
9           aux7::init();

(gdb) next
25              *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);

(gdb) disassemble /m
Dump of assembler code for function main:
7       #[entry]

8       fn main() -> ! {
9           aux7::init();
   0x08000188 <+0>:     bl      0x800019c <aux7::init>
   0x0800018c <+4>:     movw    r0, #4120       ; 0x1018
   0x08000190 <+8>:     mov.w   r1, #134217728  ; 0x8000000
   0x08000194 <+12>:    movt    r0, #18432      ; 0x4800

10
11          unsafe {
12              // A magic address!
13              const GPIOE_BSRR: u32 = 0x48001018;
14
15              // Turn on the "North" LED (red)
16              *(GPIOE_BSRR as *mut u32) = 1 << 9;
17
18              // Turn on the "East" LED (green)
19              *(GPIOE_BSRR as *mut u32) = 1 << 11;
20
21              // Turn off the "North" LED
22              *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
23
24              // Turn off the "East" LED
25              *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
=> 0x08000198 <+16>:    str     r1, [r0, #0]

26          }
27
28          loop {}
   0x0800019a <+18>:    b.n     0x800019a <main+18>

End of assembler dump.
```

The state of the LEDs didn't change this time! The `str` instruction is the one that writes a value
to the register. Our *debug* (unoptimized) program had four of them, one for each write to the
register, but the *release* (optimized) program only has one.

We can check that using `objdump`:

``` console
$ # same as cargo objdump -- -d -no-show-raw-insn -print-imm-hex -source target/thumbv7em-none-eabihf/debug/registers
$ cargo objdump --bin registers -- -d -no-show-raw-insn -print-imm-hex -source
registers:      file format ELF32-arm-little

Disassembly of section .text:
main:
; #[entry]
 8000188:       sub     sp, #0x18
; aux7::init();
 800018a:       bl      #0xbc
 800018e:       str     r0, [sp, #0x14]
 8000190:       b       #-0x2 <main+0xa>
; *(GPIOE_BSRR as *mut u32) = 1 << 9;
 8000192:       b       #-0x2 <main+0xc>
 8000194:       movw    r0, #0x1018
 8000198:       movt    r0, #0x4800
 800019c:       mov.w   r1, #0x200
 80001a0:       str     r1, [r0]
; *(GPIOE_BSRR as *mut u32) = 1 << 11;
 80001a2:       b       #-0x2 <main+0x1c>
 80001a4:       movw    r0, #0x1018
 80001a8:       movt    r0, #0x4800
 80001ac:       mov.w   r1, #0x800
 80001b0:       str     r1, [r0]
 80001b2:       movs    r0, #0x19
; *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
 80001b4:       mov     r1, r0
 80001b6:       cmp     r0, #0x9
 80001b8:       str     r1, [sp, #0x10]
 80001ba:       bvs     #0x54 <main+0x8a>
 80001bc:       b       #-0x2 <main+0x36>
 80001be:       ldr     r0, [sp, #0x10]
 80001c0:       and     r1, r0, #0x1f
 80001c4:       movs    r2, #0x1
 80001c6:       lsl.w   r1, r2, r1
 80001ca:       lsrs    r2, r0, #0x5
 80001cc:       cmp     r2, #0x0
 80001ce:       str     r1, [sp, #0xc]
 80001d0:       bne     #0x4c <main+0x98>
 80001d2:       b       #-0x2 <main+0x4c>
 80001d4:       movw    r0, #0x1018
 80001d8:       movt    r0, #0x4800
 80001dc:       ldr     r1, [sp, #0xc]
 80001de:       str     r1, [r0]
 80001e0:       movs    r0, #0x1b
; *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
 80001e2:       mov     r2, r0
 80001e4:       cmp     r0, #0xb
 80001e6:       str     r2, [sp, #0x8]
 80001e8:       bvs     #0x42 <main+0xa6>
 80001ea:       b       #-0x2 <main+0x64>
 80001ec:       ldr     r0, [sp, #0x8]
 80001ee:       and     r1, r0, #0x1f
 80001f2:       movs    r2, #0x1
 80001f4:       lsl.w   r1, r2, r1
 80001f8:       lsrs    r2, r0, #0x5
 80001fa:       cmp     r2, #0x0
 80001fc:       str     r1, [sp, #0x4]
 80001fe:       bne     #0x3a <main+0xb4>
 8000200:       b       #-0x2 <main+0x7a>
 8000202:       movw    r0, #0x1018
 8000206:       movt    r0, #0x4800
 800020a:       ldr     r1, [sp, #0x4]
 800020c:       str     r1, [r0]
; loop {}
 800020e:       b       #-0x2 <main+0x88>
 8000210:       b       #-0x4 <main+0x88>
; *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
 8000212:       movw    r0, #0x41bc
 8000216:       movt    r0, #0x800
 800021a:       bl      #0x3b28
 800021e:       trap
 8000220:       movw    r0, #0x4204
 8000224:       movt    r0, #0x800
 8000228:       bl      #0x3b1a
 800022c:       trap
; *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
 800022e:       movw    r0, #0x421c
 8000232:       movt    r0, #0x800
 8000236:       bl      #0x3b0c
 800023a:       trap
 800023c:       movw    r0, #0x4234
 8000240:       movt    r0, #0x800
 8000244:       bl      #0x3afe
 8000248:       trap
```

How do we prevent LLVM from misoptimizing our program? We use *volatile* operations instead of plain
reads/writes:

``` rust
#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
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

    loop {}
}
```

If we look at the disassembly of this new program compiled in release mode:

``` console
$ cargo objdump --bin registers --release -- -d -no-show-raw-insn -print-imm-hex -source
registers:      file format ELF32-arm-little

Disassembly of section .text:
main:
; #[entry]
 8000188:       bl      #0x22
; aux7::init();
 800018c:       movw    r0, #0x1018
 8000190:       mov.w   r1, #0x200
 8000194:       movt    r0, #0x4800
 8000198:       str     r1, [r0]
 800019a:       mov.w   r1, #0x800
 800019e:       str     r1, [r0]
 80001a0:       mov.w   r1, #0x2000000
 80001a4:       str     r1, [r0]
 80001a6:       mov.w   r1, #0x8000000
 80001aa:       str     r1, [r0]
; loop {}
 80001ac:       b       #-0x4 <main+0x24>
```

We see that the four writes (`str` instructions) are preserved. If you run it (use `stepi`), you'll
also see that behavior of the program is preserved.
