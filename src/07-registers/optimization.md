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
Dump of assembler code for function registers::main:
10      fn main() -> ! {
   0x08000188 <+0>:     push    {r7, lr}

11          aux7::init();
   0x0800018a <+2>:     bl      0x80001a4 <aux7::init>

12
13          unsafe {
14              // A magic address!
15              const GPIOE_BSRR: u32 = 0x48001018;
16
17              // Turn on the "North" LED (red)
18              *(GPIOE_BSRR as *mut u32) = 1 << 9;
19
20              // Turn on the "East" LED (green)
21              *(GPIOE_BSRR as *mut u32) = 1 << 11;
22
23              // Turn off the "North" LED
24              *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
25
26              // Turn off the "East" LED
27              *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
=> 0x0800018e <+6>:     movw    r0, #4120       ; 0x1018
   0x08000192 <+10>:    mov.w   r1, #134217728  ; 0x8000000
   0x08000196 <+14>:    movt    r0, #18432      ; 0x4800
   0x0800019a <+18>:    str     r1, [r0, #0]    ; <- 1

28          }
29
30          loop {}
   0x0800019c <+20>:    b.n     0x800019c <registers::main+20>

End of assembler dump.
```

The state of the LEDs didn't change this time! The `str` instruction is the one that writes a value
to the register. Our *debug* (unoptimized) program had four of them, one for each write to the
register, but the *release* (optimized) program only has one.

We can check that using `objdump`:

``` console
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/debug/registers
08000188 <registers::main>:
 8000188:       b580            push    {r7, lr}
 800018a:       b086            sub     sp, #24
 800018c:       f000 f86a       bl      8000264 <aux7::init>
 8000190:       9005            str     r0, [sp, #20]
 8000192:       e7ff            b.n     8000194 <registers::main+0xc>
 8000194:       e7ff            b.n     8000196 <registers::main+0xe>
 8000196:       f241 0018       movw    r0, #4120       ; 0x1018
 800019a:       f6c4 0000       movt    r0, #18432      ; 0x4800
 800019e:       f44f 7100       mov.w   r1, #512        ; 0x200
 80001a2:       6001            str     r1, [r0, #0]    ; <- 1
 80001a4:       e7ff            b.n     80001a6 <registers::main+0x1e>
 80001a6:       f241 0018       movw    r0, #4120       ; 0x1018
 80001aa:       f6c4 0000       movt    r0, #18432      ; 0x4800
 80001ae:       f44f 6100       mov.w   r1, #2048       ; 0x800
 80001b2:       6001            str     r1, [r0, #0]    ; <- 2
 80001b4:       2019            movs    r0, #25
 80001b6:       4601            mov     r1, r0
 80001b8:       2809            cmp     r0, #9
 80001ba:       9104            str     r1, [sp, #16]
 80001bc:       d62c            bvs.n   8000218 <registers::main+0x90>
 80001be:       e7ff            b.n     80001c0 <registers::main+0x38>
 80001c0:       9804            ldr     r0, [sp, #16]
 80001c2:       f000 011f       and.w   r1, r0, #31
 80001c6:       2201            movs    r2, #1
 80001c8:       fa02 f101       lsl.w   r1, r2, r1
 80001cc:       f06f 021f       mvn.w   r2, #31
 80001d0:       4210            tst     r0, r2
 80001d2:       9103            str     r1, [sp, #12]
 80001d4:       d127            bne.n   8000226 <registers::main+0x9e>
 80001d6:       e7ff            b.n     80001d8 <registers::main+0x50>
 80001d8:       f241 0018       movw    r0, #4120       ; 0x1018
 80001dc:       f6c4 0000       movt    r0, #18432      ; 0x4800
 80001e0:       9903            ldr     r1, [sp, #12]
 80001e2:       6001            str     r1, [r0, #0]    ; <- 3
 80001e4:       201b            movs    r0, #27
 80001e6:       4602            mov     r2, r0
 80001e8:       280b            cmp     r0, #11
 80001ea:       9202            str     r2, [sp, #8]
 80001ec:       d622            bvs.n   8000234 <registers::main+0xac>
 80001ee:       e7ff            b.n     80001f0 <registers::main+0x68>
 80001f0:       9802            ldr     r0, [sp, #8]
 80001f2:       f000 011f       and.w   r1, r0, #31
 80001f6:       2201            movs    r2, #1
 80001f8:       fa02 f101       lsl.w   r1, r2, r1
 80001fc:       f06f 021f       mvn.w   r2, #31
 8000200:       4210            tst     r0, r2
 8000202:       9101            str     r1, [sp, #4]
 8000204:       d11d            bne.n   8000242 <registers::main+0xba>
 8000206:       e7ff            b.n     8000208 <registers::main+0x80>
 8000208:       f241 0018       movw    r0, #4120       ; 0x1018
 800020c:       f6c4 0000       movt    r0, #18432      ; 0x4800
 8000210:       9901            ldr     r1, [sp, #4]
 8000212:       6001            str     r1, [r0, #0]    ; <- 4
 8000214:       e7ff            b.n     8000216 <registers::main+0x8e>
 8000216:       e7fe            b.n     8000216 <registers::main+0x8e>
 8000218:       f244 205c       movw    r0, #16988      ; 0x425c
 800021c:       f6c0 0000       movt    r0, #2048       ; 0x800
 8000220:       f003 fed6       bl      8003fd0 <core::panicking::panic>
 8000224:       defe            udf     #254    ; 0xfe
 8000226:       f244 20a4       movw    r0, #17060      ; 0x42a4
 800022a:       f6c0 0000       movt    r0, #2048       ; 0x800
 800022e:       f003 fecf       bl      8003fd0 <core::panicking::panic>
 8000232:       defe            udf     #254    ; 0xfe
 8000234:       f244 20bc       movw    r0, #17084      ; 0x42bc
 8000238:       f6c0 0000       movt    r0, #2048       ; 0x800
 800023c:       f003 fec8       bl      8003fd0 <core::panicking::panic>
 8000240:       defe            udf     #254    ; 0xfe
 8000242:       f244 20d4       movw    r0, #17108      ; 0x42d4
 8000246:       f6c0 0000       movt    r0, #2048       ; 0x800
 800024a:       f003 fec1       bl      8003fd0 <core::panicking::panic>
 800024e:       defe            udf     #254    ; 0xfe
```

How do we prevent LLVM from misoptimizing our program? We use *volatile* operations instead of plain
reads/writes:

``` rust
#![no_main]
#![no_std]

extern crate aux7;
#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;

use core::ptr;

entry!(main);

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

If we look at the disassemble of this new program compiled in release mode:

``` console
$ arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/release/registers
08000188 <registers::main>:
 8000188:       b580            push    {r7, lr}
 800018a:       f000 f814       bl      80001b6 <aux7::init>
 800018e:       f241 0018       movw    r0, #4120       ; 0x1018
 8000192:       f44f 7100       mov.w   r1, #512        ; 0x200
 8000196:       f6c4 0000       movt    r0, #18432      ; 0x4800
 800019a:       6001            str     r1, [r0, #0]    ; <- 1
 800019c:       f44f 6100       mov.w   r1, #2048       ; 0x800
 80001a0:       6001            str     r1, [r0, #0]    ; <- 2
 80001a2:       f04f 7100       mov.w   r1, #33554432   ; 0x2000000
 80001a6:       6001            str     r1, [r0, #0]    ; <- 3
 80001a8:       f04f 6100       mov.w   r1, #134217728  ; 0x8000000
 80001ac:       6001            str     r1, [r0, #0]    ; <- 4
 80001ae:       e7fe            b.n     80001ae <registers::main+0x26>
```

We see that the four writes (`str` instructions) are preserved. If you run it (use `stepi`), you'll
also see that behavior of the program is preserved.
