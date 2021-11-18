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
(..)
Breakpoint 1, registers::__cortex_m_rt_main_trampoline () at src/07-registers/src/main.rs:9
9       #[entry]

(gdb) continue
Continuing.

Breakpoint 3, cortex_m_rt::HardFault_ (ef=0x20009fb0)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:560
560         loop {

(gdb)
```

We tried to do an invalid operation, reading memory that doesn't exist, so the processor raised an
*exception*, a *hardware* exception.

In most cases, exceptions are raised when the processor attempts to perform an invalid operation.
Exceptions break the normal flow of a program and force the processor to execute an *exception
handler*, which is just a function/subroutine.

There are different kind of exceptions. Each kind of exception is raised by different conditions and
each one is handled by a different exception handler.

The `aux7` crate depends on the `cortex-m-rt` crate which defines a default
*hard fault* handler, named `HardFault`, that handles the "invalid memory
address" exception. `openocd.gdb` placed a breakpoint on `HardFault`; that's why
the debugger halted your program while it was executing the exception handler.
We can get more information about the exception from the debugger. Let's see:

```
(gdb) list
555     #[allow(unused_variables)]
556     #[doc(hidden)]
557     #[link_section = ".HardFault.default"]
558     #[no_mangle]
559     pub unsafe extern "C" fn HardFault_(ef: &ExceptionFrame) -> ! {
560         loop {
561             // add some side effect to prevent this from turning into a UDF instruction
562             // see rust-lang/rust#28728 for details
563             atomic::compiler_fence(Ordering::SeqCst);
564         }
```

`ef` is a snapshot of the program state right before the exception occurred. Let's inspect it:

```
(gdb) print/x *ef
$1 = cortex_m_rt::ExceptionFrame {
  r0: 0x48001800,
  r1: 0x80036b0,
  r2: 0x1,
  r3: 0x80000000,
  r12: 0xb,
  lr: 0x800020d,
  pc: 0x8001750,
  xpsr: 0xa1000200
}
```

There are several fields here but the most important one is `pc`, the Program Counter register.
The address in this register points to the instruction that generated the exception. Let's
disassemble the program around the bad instruction.

```
(gdb) disassemble /m ef.pc
Dump of assembler code for function core::ptr::read_volatile<u32>:
1046    pub unsafe fn read_volatile<T>(src: *const T) -> T {
   0x0800174c <+0>:     sub     sp, #12
   0x0800174e <+2>:     str     r0, [sp, #4]

1047        if cfg!(debug_assertions) && !is_aligned_and_not_null(src) {
1048            // Not panicking to keep codegen impact smaller.
1049            abort();
1050        }
1051        // SAFETY: the caller must uphold the safety contract for `volatile_load`.
1052        unsafe { intrinsics::volatile_load(src) }
   0x08001750 <+4>:     ldr     r0, [r0, #0]
   0x08001752 <+6>:     str     r0, [sp, #8]
   0x08001754 <+8>:     ldr     r0, [sp, #8]
   0x08001756 <+10>:    str     r0, [sp, #0]
   0x08001758 <+12>:    b.n     0x800175a <core::ptr::read_volatile<u32>+14>

1053    }
   0x0800175a <+14>:    ldr     r0, [sp, #0]
   0x0800175c <+16>:    add     sp, #12
   0x0800175e <+18>:    bx      lr

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
(gdb) print/x *ef
$1 = cortex_m_rt::ExceptionFrame {
  r0: 0x48001800,
  r1: 0x80036b0,
  r2: 0x1,
  r3: 0x80000000,
  r12: 0xb,
  lr: 0x800020d,
  pc: 0x8001750,
  xpsr: 0xa1000200
}
```

`r0` contains the value `0x4800_1800` which is the invalid address we called the `read_volatile`
function with.
