# `0xBAAAAAAD` address

Not all the peripheral memory can be accessed. Look at this program.

``` rust
#![no_main]
#![no_std]

extern crate aux7;
#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;

entry!(main);

fn main() -> ! {
    aux7::init();

    unsafe {
        core::ptr::read_volatile(0x4800_1800 as *const u32);
    }

    loop {}
}
```

This address is close to the `GPIOE_BSRR` address we used before but this address is *invalid*.
Invalid in the sense that there's no register at this address.

Now, let's try it.

``` console
$ cargo run

(gdb) continue
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
__bkpt () at asm/bkpt.s:3
3         bkpt
```

We tried to do an invalid operation, reading memory that doesn't exist, so the processor raised an
*exception*, a *hardware* exception.

In most cases, exceptions are raised when the processor attempts to perform an invalid operation.
Exceptions break the normal flow of a program and force the processor to execute an *exception
handler*, which is just a function/subroutine.

There are different kind of exceptions. Each kind of exception is raised by different conditions and
each one is handled by a different exception handler.

The `aux7` crate defines a *hard fault* handler that handles the  "invalid memory address"
execption. This exception handler triggered a breakpoint (via `asm::bkpt()`); that's why the
debugger halted your program while it was executing the exception handler. We can get more
information about the exception from the debugger. Let's see:

```
(gdb) finish

(gdb) list
44      }
45
46      exception!(HardFault, hard_fault);
47
48      fn hard_fault(_ef: &ExceptionFrame) -> ! {
49          asm::bkpt();
50
51          loop {}
52      }
53
```

`_ef` is a snapshot of the program state right before the exception occurred. Let's inspect it:

```
(gdb) print/x *_ef
$1 = cortex_m_rt::ExceptionFrame {
  r0: 0x48001800,
  r1: 0x48001800,
  r2: 0xb,
  r3: 0xc,
  r12: 0xd,
  lr: 0x80001a1,
  pc: 0x8002b1a,
  xpsr: 0x1000000
}
```

There are several fields here but the most important one is `pc`, the Program Counter register.
The address in this register points to the instruction that generated the exception. Let's
disassemble the program around the bad instruction.

```
(gdb) disassemble /m _ef.pc
Dump of assembler code for function core::ptr::read_volatile:
452     pub unsafe fn read_volatile<T>(src: *const T) -> T {
   0x08002b12 <+0>:     sub     sp, #16
   0x08002b14 <+2>:     mov     r1, r0
   0x08002b16 <+4>:     str     r0, [sp, #8]

453         intrinsics::volatile_load(src)
   0x08002b18 <+6>:     ldr     r0, [sp, #8]
=> 0x08002b1a <+8>:     ldr     r0, [r0, #0]
   0x08002b1c <+10>:    str     r0, [sp, #12]
   0x08002b1e <+12>:    ldr     r0, [sp, #12]
   0x08002b20 <+14>:    str     r1, [sp, #4]
   0x08002b22 <+16>:    str     r0, [sp, #0]
   0x08002b24 <+18>:    b.n     0x8002b26 <core::ptr::read_volatile+20>

454     }
   0x08002b26 <+20>:    ldr     r0, [sp, #0]
   0x08002b28 <+22>:    add     sp, #16
   0x08002b2a <+24>:    bx      lr

End of assembler dump.
```

The exception was caused by the `ldr r0, [r0, #0]` instruction, a read instruction. The instruction
tried to read the memory at the address indicated by the `r0` register. By the way, `r0` is a CPU
(processor) register not a memory mapped register; it doesn't have an associated address like, say,
`GPIO_BSRR`.

Wouldn't it be nice if we could check what the value of the `r0` register was right at the instant
when the exception was raised? Well, we already did! The `r0` field in the `_ef` value we printed
before is the value of `r0` register had when the exception was raised. Here it is again:

```
(gdb) p/x *_ef
$1 = cortex_m::exception::ExceptionFrame {
  r0: 0x48001800,
  r1: 0x48001800,
  r2: 0x0,
  r3: 0x0,
  r12: 0x0,
  lr: 0x80001ff,
  pc: 0x8000276,
  xpsr: 0x41000000
}
```

`r0` contains the value `0x4800_1800` which is the invalid address we called the `read_volatile`
function with.
