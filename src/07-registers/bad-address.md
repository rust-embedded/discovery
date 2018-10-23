# `0xBAAAAAAD` address

Not all the peripheral memory can be accessed. Look at this program.

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
        ptr::read_volatile(0x4800_1800 as *const u32);
    }

    loop {}
}
```

This address is close to the `GPIOE_BSRR` address we used before but this address is *invalid*.
Invalid in the sense that there's no register at this address.

Now, let's try it.

``` console
$ cargo run
Breakpoint 3, main () at src/07-registers/src/main.rs:9
9           aux7::init();

(gdb) continue
Continuing.

Breakpoint 2, UserHardFault_ (ef=0x10001fc0)
    at $REGISTRY/cortex-m-rt-0.6.3/src/lib.rs:535
535         loop {
```

We tried to do an invalid operation, reading memory that doesn't exist, so the processor raised an
*exception*, a *hardware* exception.

In most cases, exceptions are raised when the processor attempts to perform an invalid operation.
Exceptions break the normal flow of a program and force the processor to execute an *exception
handler*, which is just a function/subroutine.

There are different kind of exceptions. Each kind of exception is raised by different conditions and
each one is handled by a different exception handler.

The `aux7` crate depends on the `cortex-m-rt` crate which defines a default
*hard fault* handler, named `UserHardFault`, that handles the "invalid memory
address" exception. `openocd.gdb` placed a breakpoint on `HardFault`; that's why
the debugger halted your program while it was executing the exception handler.
We can get more information about the exception from the debugger. Let's see:

```
(gdb) list
530
531     #[allow(unused_variables)]
532     #[doc(hidden)]
533     #[no_mangle]
534     pub unsafe extern "C" fn UserHardFault_(ef: &ExceptionFrame) -> ! {
535         loop {
536             // add some side effect to prevent this from turning into a UDF instruction
537             // see rust-lang/rust#28728 for details
538             atomic::compiler_fence(Ordering::SeqCst);
539         }
```

`ef` is a snapshot of the program state right before the exception occurred. Let's inspect it:

```
(gdb) print/x *ef
$1 = cortex_m_rt::ExceptionFrame {
  r0: 0x48001800,
  r1: 0x48001800,
  r2: 0xb,
  r3: 0xc,
  r12: 0xd,
  lr: 0x800019f,
  pc: 0x80028d6,
  xpsr: 0x1000000
}
```

There are several fields here but the most important one is `pc`, the Program Counter register.
The address in this register points to the instruction that generated the exception. Let's
disassemble the program around the bad instruction.

```
(gdb) disassemble /m ef.pc
Dump of assembler code for function core::ptr::read_volatile:
471     /checkout/src/libcore/ptr.rs: No such file or directory.
   0x080028ce <+0>:     sub     sp, #16
   0x080028d0 <+2>:     mov     r1, r0
   0x080028d2 <+4>:     str     r0, [sp, #8]

472     in /checkout/src/libcore/ptr.rs
   0x080028d4 <+6>:     ldr     r0, [sp, #8]
   0x080028d6 <+8>:     ldr     r0, [r0, #0]
   0x080028d8 <+10>:    str     r0, [sp, #12]
   0x080028da <+12>:    ldr     r0, [sp, #12]
   0x080028dc <+14>:    str     r1, [sp, #4]
   0x080028de <+16>:    str     r0, [sp, #0]
   0x080028e0 <+18>:    b.n     0x80028e2 <core::ptr::read_volatile+20>

473     in /checkout/src/libcore/ptr.rs
   0x080028e2 <+20>:    ldr     r0, [sp, #0]
   0x080028e4 <+22>:    add     sp, #16
   0x080028e6 <+24>:    bx      lr

End of assembler dump.
```

The exception was caused by the `ldr r0, [r0, #0]` instruction, a read instruction. The instruction
tried to read the memory at the address indicated by the `r0` register. By the way, `r0` is a CPU
(processor) register not a memory mapped register; it doesn't have an associated address like, say,
`GPIO_BSRR`.

Wouldn't it be nice if we could check what the value of the `r0` register was right at the instant
when the exception was raised? Well, we already did! The `r0` field in the `ef` value we printed
before is the value of `r0` register had when the exception was raised. Here it is again:

```
(gdb) p/x *ef
$1 = cortex_m_rt::ExceptionFrame {
  r0: 0x48001800,
  r1: 0x48001800,
  r2: 0xb,
  r3: 0xc,
  r12: 0xd,
  lr: 0x800019f,
  pc: 0x80028d6,
  xpsr: 0x1000000
}
```

`r0` contains the value `0x4800_1800` which is the invalid address we called the `read_volatile`
function with.
