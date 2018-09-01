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
$ # same as cargo objdump -- -d -no-show-raw-insn target/thumbv7em-none-eabihf/debug/registers
$ cargo objdump --bin registers -- -d -no-show-raw-insn
registers::main::h92bcf844b62ba8a0:
; fn main() -> ! {
 8000188:       push    {r7, lr}
 800018a:       sub     sp, #24
; aux7::init();
 800018c:       bl      #212
 8000190:       str     r0, [sp, #20]
 8000192:       b       #-2 <registers::main::h92bcf844b62ba8a0+0xc>
; *(GPIOE_BSRR as *mut u32) = 1 << 9;
 8000194:       b       #-2 <registers::main::h92bcf844b62ba8a0+0xe>
 8000196:       movw    r0, #4120
 800019a:       movt    r0, #18432
 800019e:       mov.w   r1, #512
 80001a2:       str     r1, [r0]
; *(GPIOE_BSRR as *mut u32) = 1 << 11;
 80001a4:       b       #-2 <registers::main::h92bcf844b62ba8a0+0x1e>
 80001a6:       movw    r0, #4120
 80001aa:       movt    r0, #18432
 80001ae:       mov.w   r1, #2048
 80001b2:       str     r1, [r0]
; *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
 80001b4:       movs    r0, #25
 80001b6:       mov     r1, r0
 80001b8:       cmp     r0, #9
 80001ba:       str     r1, [sp, #16]
 80001bc:       bvs     #88 <registers::main::h92bcf844b62ba8a0+0x90>
 80001be:       b       #-2 <registers::main::h92bcf844b62ba8a0+0x38>
 80001c0:       ldr     r0, [sp, #16]
 80001c2:       and     r1, r0, #31
 80001c6:       movs    r2, #1
 80001c8:       lsl.w   r1, r2, r1
 80001cc:       mvn     r2, #31
 80001d0:       tst     r0, r2
 80001d2:       str     r1, [sp, #12]
 80001d4:       bne     #78 <registers::main::h92bcf844b62ba8a0+0x9e>
 80001d6:       b       #-2 <registers::main::h92bcf844b62ba8a0+0x50>
 80001d8:       movw    r0, #4120
 80001dc:       movt    r0, #18432
 80001e0:       ldr     r1, [sp, #12]
 80001e2:       str     r1, [r0]
; *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
 80001e4:       movs    r0, #27
 80001e6:       mov     r2, r0
 80001e8:       cmp     r0, #11
 80001ea:       str     r2, [sp, #8]
 80001ec:       bvs     #68 <registers::main::h92bcf844b62ba8a0+0xac>
 80001ee:       b       #-2 <registers::main::h92bcf844b62ba8a0+0x68>
 80001f0:       ldr     r0, [sp, #8]
 80001f2:       and     r1, r0, #31
 80001f6:       movs    r2, #1
 80001f8:       lsl.w   r1, r2, r1
 80001fc:       mvn     r2, #31
 8000200:       tst     r0, r2
 8000202:       str     r1, [sp, #4]
 8000204:       bne     #58 <registers::main::h92bcf844b62ba8a0+0xba>
 8000206:       b       #-2 <registers::main::h92bcf844b62ba8a0+0x80>
 8000208:       movw    r0, #4120
 800020c:       movt    r0, #18432
 8000210:       ldr     r1, [sp, #4]
 8000212:       str     r1, [r0]
; loop {}
 8000214:       b       #-2 <registers::main::h92bcf844b62ba8a0+0x8e>
 8000216:       b       #-4 <registers::main::h92bcf844b62ba8a0+0x8e>
; *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
 8000218:       movw    r0, #16988
 800021c:       movt    r0, #2048
 8000220:       bl      #16112
 8000224:       trap
 8000226:       movw    r0, #17060
 800022a:       movt    r0, #2048
 800022e:       bl      #16098
 8000232:       trap
; *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
 8000234:       movw    r0, #17084
 8000238:       movt    r0, #2048
 800023c:       bl      #16084
 8000240:       trap
 8000242:       movw    r0, #17108
 8000246:       movt    r0, #2048
 800024a:       bl      #16070
 800024e:       trap
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

If we look at the disassembly of this new program compiled in release mode:

``` console
$ cargo objdump --bin registers --release -- -source -no-show-raw-insn
registers::main::h3fb012c2979103e9:
 8000188:       push    {r7, lr}
 800018a:       bl      #40
 800018e:       movw    r0, #4120
 8000192:       mov.w   r1, #512
 8000196:       movt    r0, #18432
 800019a:       str     r1, [r0]
 800019c:       mov.w   r1, #2048
 80001a0:       str     r1, [r0]
 80001a2:       mov.w   r1, #33554432
 80001a6:       str     r1, [r0]
 80001a8:       mov.w   r1, #134217728
 80001ac:       str     r1, [r0]
 80001ae:       b       #-4 <registers::main::h3fb012c2979103e9+0x26>
```

We see that the four writes (`str` instructions) are preserved. If you run it (use `stepi`), you'll
also see that behavior of the program is preserved.
