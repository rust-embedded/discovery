# `0xBAAAAAAD` address

Not all the peripheral memory can be accessed. Look at this program.

``` rust
pub fn main() -> ! {
    unsafe {
        ptr::read_volatile(0x4800_1800 as *const u32);
    }

    loop {}
}
```

This address is close to the `GPIOE_BSRR` address we used before but this
address is "invalid". Invalid in the sense that there's no register at this
address.

Now, let's try it. Make sure you have `itmdump` running.

After executing the `read_volatile` statement, you should see this in
`itmdump`'s console:

```
# itmdump's console
EXCEPTION HardFault @ PC=0x0800022a
```

We tried to do an invalid operation, reading memory that doesn't exist, so the
processor raised an *exception*, a *hardware* exception.

In most cases, exceptions are raised when the processor attempts to perform an
invalid operation. Exceptions break the normal flow of a program and force the
processor to execute an *exception handler*, which is just a
function/subroutine.

There are different kind of exceptions. Each kind of exception is raised by
different conditions and each one is handled by a different exception handler.

The `pg` crate provides a catch-all exception handler and that's what the
processor executed upon encountering the "invalid memory address" exception.
That handler is also what caused the `EXCEPTION` line to be printed to the ITM.

This `EXCEPTION` line provides information about the exception. It tells us its
kind: `HardFault` and which instruction caused the exception: the one at address
`0x0800022a`.

The exception handler also triggered a breakpoint (via `bkpt!()`) so the
debugger should have halted your program while it was executing the exception
handler.

Let's disassemble the program around the bad instruction.

```
(gdb) disassemble /m 0x0800022a
Dump of assembler code for function core::ptr::read_volatile<u32>:
213     pub unsafe fn read_volatile<T>(src: *const T) -> T {
   0x0800021c <+0>:     sub     sp, #20
   0x0800021e <+2>:     mov     r1, r0
   0x08000220 <+4>:     str     r0, [sp, #16]
   0x08000222 <+6>:     str     r1, [sp, #4]
   0x08000224 <+8>:     b.n     0x8000226 <core::ptr::read_volatile<u32>+10>
   0x08000226 <+10>:    ldr     r0, [sp, #16]
   0x08000228 <+12>:    str     r0, [sp, #12]
   0x0800022a <+14>:    ldr     r0, [r0, #0] <--
   0x0800022c <+16>:    str     r0, [sp, #8]
   0x08000232 <+22>:    ldr     r0, [sp, #0]
   0x08000234 <+24>:    add     sp, #20
   0x08000236 <+26>:    bx      lr

214         intrinsics::volatile_load(src)
   0x0800022e <+18>:    str     r0, [sp, #0]
   0x08000230 <+20>:    b.n     0x8000232 <core::ptr::read_volatile<u32>+22>
```

The exception was caused by a `ldr` instruction, a read instruction. The
instruction tried to read the memory at the address indicated by the `r0`
register. By the way, `r0` is a CPU (processor) register not a microcontroller
register.

Wouldn't it be nice if we could check what the value of the `r0` register was
right at the instant when the exception was raised? Well, we can!

If you looked carefully at the GDB output right when the exception was hit, you
probably saw this:

```
Program received signal SIGTRAP, Trace/breakpoint trap.
f3::exception::default_handler (sf=0x20009fa0) at $F3/src/exception.rs:82
```

The exception handler we are in right now was called with an argument. Let's
inspect that argument:

```
(gdb) p sf
$5 = (cortex_m::StackFrame *) 0x20009fa8

(gdb) p/x *sf
$4 = cortex_m::StackFrame {
  r0: 0x48001800,
  r1: 0x48001800,
  r2: 0xd,
  r3: 0x40013800,
  r12: 0x2,
  lr: 0x8000217,
  pc: 0x80001f4,
  xpsr: 0x41000200
}
```

This `StackFrame` struct contains the state of your program right before the
exception was hit. There's an `r0` field in it. That's the value of `r0` right
before the exception was raised. It contains the value `0x4800_1800` which is
the invalid address we fed to the `read_volatile` function.
