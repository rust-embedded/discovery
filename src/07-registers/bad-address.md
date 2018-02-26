# `0xBAAAAAAD` address

Not all the peripheral memory can be accessed. Look at this program.

``` rust
fn main() {
    unsafe {
        core::ptr::read_volatile(0x4800_1800 as *const u32);
    }
}
```

This address is close to the `GPIOE_BSRR` address we used before but this address is *invalid*.
Invalid in the sense that there's no register at this address.

Now, let's try it.

``` console
$ xargo run

(gdb) continue
cortex_m_rt::default_handler (ef=0x10001fb8) at $REGISTRY/cortex-m-rt-0.3.9/src/lib.rs:452
452         asm::bkpt();
```

We tried to do an invalid operation, reading memory that doesn't exist, so the processor raised an
*exception*, a *hardware* exception.

In most cases, exceptions are raised when the processor attempts to perform an invalid operation.
Exceptions break the normal flow of a program and force the processor to execute an *exception
handler*, which is just a function/subroutine.

There are different kind of exceptions. Each kind of exception is raised by different conditions and
each one is handled by a different exception handler.

The `cortex-m-rt` crate, which all of our programs link to, provides a catch-all exception handler
and that's what the processor executed upon encountering the "invalid memory address" exception.

The default exception handler triggered a breakpoint (via `asm::bkpt()`); that's why the debugger
halted your program while it was executing the exception handler. We can get more information about
the exception from the debugger. Let's see:

```
(gdb) list
447     /// That exception frame is a snapshot of the program state right before the
448     /// exception occurred.
449     #[allow(unused_variables)]
450     #[cfg(target_arch = "arm")]
451     extern "C" fn default_handler(ef: &ExceptionFrame) -> ! {
452         asm::bkpt();
453
454         loop {}
```

`ef` is a snapshot of the program state right before the exception occurred. Let's inspect it:

```
(gdb) print/x *ef
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

There are several registers here but the most important one is `pc`, the Program Counter register.
The address in this register points to the instruction that generated the exception. Let's
disassemble the program around the bad instruction.

```
(gdb) disassemble /m ef.pc
Dump of assembler code for function core::ptr::read_volatile:
453     pub unsafe fn read_volatile<T>(src: *const T) -> T {
   0x0800026e <+0>:     sub     sp, #16
   0x08000270 <+2>:     mov     r1, r0
   0x08000272 <+4>:     str     r0, [sp, #8]

454         intrinsics::volatile_load(src)
   0x08000274 <+6>:     ldr     r0, [sp, #8]
=> 0x08000276 <+8>:     ldr     r0, [r0, #0]
   0x08000278 <+10>:    str     r0, [sp, #12]
   0x0800027a <+12>:    ldr     r0, [sp, #12]
   0x0800027c <+14>:    str     r1, [sp, #4]
   0x0800027e <+16>:    str     r0, [sp, #0]
   0x08000280 <+18>:    b.n     0x8000282 <core::ptr::read_volatile+20>

455     }
   0x08000282 <+20>:    ldr     r0, [sp, #0]
   0x08000284 <+22>:    add     sp, #16
   0x08000286 <+24>:    bx      lr

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
