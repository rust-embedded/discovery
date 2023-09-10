# `0xBAAAAAAD` 地址

并非所有外围存储器都可以访问。看看这个程序。

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

此地址与我们之前使用的`GPIOE_BSRR`地址接近，但此地址*无效*。

现在，让我们试试看。

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

我们试图执行无效操作，读取不存在的内存，因此处理器引发了一个*异常*，一个*硬件*异常。

在大多数情况下，当处理器尝试执行无效操作时会引发异常。异常打破了程序的正常流程，迫使处理器执行
*异常处理程序*，这只是一个函数/子例程。

有不同类型的例外。每种异常都由不同的条件引发，每种异常由不同的异常处理程序处理。

`aux7` crate 依赖于`cortex-m-rt` crate，它定义了一个名为`HardFault`的默认*硬故障*处理程序，用于处理"无效内存地址"异常。
`openocd.gdb`在`HardFault`上放置了一个断点; 这就是调试器在执行异常处理程序时暂停程序的原因。
我们可以从调试器获取有关异常的更多信息。让我们看看：

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

`ef`是发生异常之前程序状态的快照。让我们检查一下：

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

这里有几个字段，但最重要的是`pc`，即程序计数器寄存器。此寄存器中的地址指向生成异常的指令。
让我们围绕坏指令来拆解程序。

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

异常是由`ldr r0, [r0, #0]`指令（读取指令）引起的。该指令试图读取`r0`寄存器指示的地址处的内存。
顺便说一句，`r0`是CPU（处理器）寄存器，而不是内存映射寄存器；它没有关联的地址，比如`GPIO_BSRR`。

如果我们能够在引发异常的瞬间检查`r0`寄存器的值是正确的，这不是很好吗？嗯，我们已经做到了！我们
之前打印的`ef`值中的`r0`字段是引发异常时`r0`寄存器的值。这里又是：

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

`r0`包含值`0x4800_1800`，这是我们调用`read_volatile`函数时使用的无效地址。
