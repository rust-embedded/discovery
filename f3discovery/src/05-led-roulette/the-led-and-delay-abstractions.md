# `Led` 和 `Delay` 抽象概念

现在，我将介绍两个高级抽象，我们将使用它们来实现LED轮盘应用程序。

auxiliary crate，`aux5`，公开了一个名为`init`的初始化函数。
调用此函数时，返回两个打包在元组中的值：`Delay`值和`LedArray`值。

`Delay`可以用于在指定的毫秒数内阻止程序。

`LedArray`是由八个`Led`组成的数组。每个`Led`代表F3板上的一个Led，并显示两种方法：
`on`和`off`分别用于打开或关闭Led。

让我们通过修改起始代码来尝试这两种抽象，如下所示：

``` rust
{{#include examples/the-led-and-delay-abstractions.rs}}
```

现在构建它：
``` console
cargo build
```

> **注意**：在开始GDB会话*之前*，可能忘记重建程序；这种遗漏可能会导致非常混乱的调试会话。
> 为了避免这个问题，你可以只调用`cargo run`而不是`cargo build`。`cargo run`命令将构建并
> 启动调试会话，确保您永远不会忘记重新编译程序。

现在，我们将运行并重复上一节中的闪烁过程，但使用新程序。我会让你输入`cargo run`，*这将很快变得容易*。 :)

> **注意**：不要忘记在单独的终端上启动```openocd``` (调试器)。否则，`target remote :3333`将无法工作！

``` console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `arm-none-eabi-gdb -q ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette`
Reading symbols from ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette...

(gdb) target remote :3333
Remote debugging using :3333
led_roulette::__cortex_m_rt_main_trampoline () at ~/embedded-discovery/src/05-led-roulette/src/main.rs:7
7       #[entry]

(gdb) load
Loading section .vector_table, size 0x194 lma 0x8000000
Loading section .text, size 0x52c0 lma 0x8000194
Loading section .rodata, size 0xb50 lma 0x8005454
Start address 0x08000194, load size 24484
Transfer rate: 21 KB/sec, 6121 bytes/write.

(gdb) break main
Breakpoint 1 at 0x8000202: file ~/embedded-discovery/src/05-led-roulette/src/main.rs, line 7.
Note: automatically using hardware breakpoints for read-only addresses.

(gdb) continue
Continuing.

Breakpoint 1, led_roulette::__cortex_m_rt_main_trampoline ()
    at ~/embedded-discovery/src/05-led-roulette/src/main.rs:7
7       #[entry]

(gdb) step
led_roulette::__cortex_m_rt_main () at ~/embedded-discovery/src/05-led-roulette/src/main.rs:9
9           let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

(gdb)
```

好的，让我们一步一步看代码。这次，我们将使用`next`命令而不是`step`。不同的是，`next`命令将跳过函数调用，而不是进入它们内部。
```
(gdb) next
11          let half_period = 500_u16;

(gdb) next
13          loop {

(gdb) next
14              leds[0].on().ok();

(gdb) next
15              delay.delay_ms(half_period);
```

执行`leds[0].on().ok()` 语句后，您应该看到一个红色LED，指向北方，打开。

让我们继续浏览该程序：

```
(gdb) next
17              leds[0].off().ok();

(gdb) next
18              delay.delay_ms(half_period);
```

`delay_ms`调用将阻止程序半秒，但您可能没有注意到，因为`next`时间才能执行。然而，在跨过
`leds[0].off()`语句，您应该看到红色LED熄灭。

你已经可以猜到这个程序的作用了。使用`continue`命令让它不间断地运行。

```
(gdb) continue
Continuing.
```

现在，让我们做一些更有趣的事情。我们将使用GDB修改程序的行为。

首先，让我们按`Ctrl+C`停止无限循环。你很可能会在`Led::on`，`Led::off` 或`delay_ms`中的某个位置结束：

```
^C
Program received signal SIGINT, Interrupt.
0x08003434 in core::ptr::read_volatile<u32> (src=0xe000e010)
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1053
```

在我的例子中，程序在`read_volatile`函数中停止了执行。GDB输出显示了一些有趣的信息：
`core::ptr::read_volatile (src=0xe000e010)`。这意味着该函数来自`core` crate，并且是用参数`src = 0xe000e010`调用的。

正如您所知，显示函数参数的更明确的方法是使用`info args`命令：

```
(gdb) info args
src = 0xe000e010
```

无论程序在何处停止，您都可以查看`backtrace`命令(简称`bt`) 的输出，了解它是如何停止的：

```
(gdb) backtrace
#0  0x08003434 in core::ptr::read_volatile<u32> (src=0xe000e010)
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1053
#1  0x08002d66 in vcell::VolatileCell<u32>::get<u32> (self=0xe000e010) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/vcell-0.1.3/src/lib.rs:33
#2  volatile_register::RW<u32>::read<u32> (self=0xe000e010) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/volatile-register-0.2.0/src/lib.rs:75
#3  cortex_m::peripheral::SYST::has_wrapped (self=0x20009fa4)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.6.4/src/peripheral/syst.rs:136
#4  0x08003004 in stm32f3xx_hal::delay::{{impl}}::delay_us (self=0x20009fa4, us=500000)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/stm32f3xx-hal-0.5.0/src/delay.rs:58
#5  0x08002f3e in stm32f3xx_hal::delay::{{impl}}::delay_ms (self=0x20009fa4, ms=500)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/stm32f3xx-hal-0.5.0/src/delay.rs:32
#6  0x08002f80 in stm32f3xx_hal::delay::{{impl}}::delay_ms (self=0x20009fa4, ms=500)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/stm32f3xx-hal-0.5.0/src/delay.rs:38
#7  0x0800024c in led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:15
#8  0x08000206 in led_roulette::__cortex_m_rt_main_trampoline () at src/05-led-roulette/src/main.rs:7
```

`backtrace`将打印从当前函数到main的函数调用跟踪。

回到我们的话题。要做我们要做的事情，首先，我们必须返回`main`函数。我们可以使用`finish`命令来完成。
该命令恢复程序执行，并在程序从当前函数返回后立即再次停止。我们得打调用几次。

```
(gdb) finish
Run till exit from #0  0x08003434 in core::ptr::read_volatile<u32> (src=0xe000e010)
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1053
cortex_m::peripheral::SYST::has_wrapped (self=0x20009fa4)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.6.4/src/peripheral/syst.rs:136
136             self.csr.read() & SYST_CSR_COUNTFLAG != 0
Value returned is $1 = 5

(..)

(gdb) finish
Run till exit from #0  0x08002f3e in stm32f3xx_hal::delay::{{impl}}::delay_ms (self=0x20009fa4, ms=500)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/stm32f3xx-hal-0.5.0/src/delay.rs:32
0x08002f80 in stm32f3xx_hal::delay::{{impl}}::delay_ms (self=0x20009fa4, ms=500)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/stm32f3xx-hal-0.5.0/src/delay.rs:38
38              self.delay_ms(u32(ms));

(gdb) finish
Run till exit from #0  0x08002f80 in stm32f3xx_hal::delay::{{impl}}::delay_ms (self=0x20009fa4, ms=500)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/stm32f3xx-hal-0.5.0/src/delay.rs:38
0x0800024c in led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:15
15              delay.delay_ms(half_period);
```

我们又回到了`main`。这里有一个局部变量：`half_period`

```
(gdb) print half_period
$3 = 500
```

现在，我们将使用`set`命令修改此变量：

```
(gdb) set half_period = 100

(gdb) print half_period
$5 = 100
```

如果您使用`continue`命令让程序再次自由运行，您**可能**会看到LED现在将以更快的速度闪烁，
但更可能的是闪烁速度没有改变。**发生了什么？**

让我们用`Ctrl+C`停止程序，然后在`main:14`处设置断点。
``` console
(gdb) continue
Continuing.
^C
Program received signal SIGINT, Interrupt.
core::cell::UnsafeCell<u32>::get<u32> (self=0x20009fa4)
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1711
1711        pub const fn get(&self) -> *mut T {
```

然后在`main.rs:14`上设置断点。并`continue`

``` console
(gdb) break main.rs:14
Breakpoint 2 at 0x8000236: file src/05-led-roulette/src/main.rs, line 14.
(gdb) continue
Continuing.

Breakpoint 2, led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:14
14              leds[0].on().ok();
```

现在打开终端窗口，其长约80行，宽约170个字符（如果可能）。
> **注意**：如果你不能打开那么大的终端，没问题，你会看到`--Type <RET> for more, q to quit, c to continue without paging--`
> 所以只需输入return，直到看到`(gdb)`提示符。然后滚动终端窗口以查看结果。

``` console
(gdb) disassemble /m
Dump of assembler code for function _ZN12led_roulette18__cortex_m_rt_main17h51e7c3daad2af251E:
8       fn main() -> ! {
   0x08000208 <+0>:     push    {r7, lr}
   0x0800020a <+2>:     mov     r7, sp
   0x0800020c <+4>:     sub     sp, #64 ; 0x40
   0x0800020e <+6>:     add     r0, sp, #32

9           let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
   0x08000210 <+8>:     bl      0x8000302 <aux5::init>
   0x08000214 <+12>:    b.n     0x8000216 <led_roulette::__cortex_m_rt_main+14>
   0x08000216 <+14>:    add     r0, sp, #32
   0x08000218 <+16>:    add     r1, sp, #4
   0x0800021a <+18>:    ldmia.w r0, {r2, r3, r4, r12, lr}
   0x0800021e <+22>:    stmia.w r1, {r2, r3, r4, r12, lr}
   0x08000222 <+26>:    ldr     r0, [sp, #52]   ; 0x34
   0x08000224 <+28>:    ldr     r1, [sp, #56]   ; 0x38
   0x08000226 <+30>:    str     r1, [sp, #28]
   0x08000228 <+32>:    str     r0, [sp, #24]
   0x0800022a <+34>:    mov.w   r0, #500        ; 0x1f4

10
11          let half_period = 500_u16;
   0x0800022e <+38>:    strh.w  r0, [r7, #-2]

12
13          loop {
   0x08000232 <+42>:    b.n     0x8000234 <led_roulette::__cortex_m_rt_main+44>
   0x08000234 <+44>:    add     r0, sp, #24
   0x08000268 <+96>:    b.n     0x8000234 <led_roulette::__cortex_m_rt_main+44>

14              leds[0].on().ok();
=> 0x08000236 <+46>:    bl      0x80001ec <switch_hal::output::{{impl}}::on<stm32f3xx_hal::gpio::gpioe::PEx<stm32f3xx_hal::gpio::Output<stm32f3xx_hal::gpio::PushPull>>>>
   0x0800023a <+50>:    b.n     0x800023c <led_roulette::__cortex_m_rt_main+52>
   0x0800023c <+52>:    bl      0x8000594 <core::result::Result<(), core::convert::Infallible>::ok<(),core::convert::Infallible>>
   0x08000240 <+56>:    b.n     0x8000242 <led_roulette::__cortex_m_rt_main+58>
   0x08000242 <+58>:    add     r0, sp, #4
   0x08000244 <+60>:    mov.w   r1, #500        ; 0x1f4

15              delay.delay_ms(half_period);
   0x08000248 <+64>:    bl      0x8002f5c <stm32f3xx_hal::delay::{{impl}}::delay_ms>
   0x0800024c <+68>:    b.n     0x800024e <led_roulette::__cortex_m_rt_main+70>
   0x0800024e <+70>:    add     r0, sp, #24

16
17              leds[0].off().ok();
   0x08000250 <+72>:    bl      0x800081a <switch_hal::output::{{impl}}::off<stm32f3xx_hal::gpio::gpioe::PEx<stm32f3xx_hal::gpio::Output<stm32f3xx_hal::gpio::PushPull>>>>
   0x08000254 <+76>:    b.n     0x8000256 <led_roulette::__cortex_m_rt_main+78>
   0x08000256 <+78>:    bl      0x8000594 <core::result::Result<(), core::convert::Infallible>::ok<(),core::convert::Infallible>>
   0x0800025a <+82>:    b.n     0x800025c <led_roulette::__cortex_m_rt_main+84>
   0x0800025c <+84>:    add     r0, sp, #4
   0x0800025e <+86>:    mov.w   r1, #500        ; 0x1f4

18              delay.delay_ms(half_period);
   0x08000262 <+90>:    bl      0x8002f5c <stm32f3xx_hal::delay::{{impl}}::delay_ms>
   0x08000266 <+94>:    b.n     0x8000268 <led_roulette::__cortex_m_rt_main+96>

End of assembler dump.
```

在上面的转储中，delay没有改变的原因是编译器认识到half_period没有改变，而是在delay的两个地方。
`delay.delay_ms(half_period);`被称为`mov.w r1, #500`。所以改变`half_period`的值没有任何作用！

``` console
   0x08000244 <+60>:    mov.w   r1, #500        ; 0x1f4

15              delay.delay_ms(half_period);
   0x08000248 <+64>:    bl      0x8002f5c <stm32f3xx_hal::delay::{{impl}}::delay_ms>

(..)

   0x0800025e <+86>:    mov.w   r1, #500        ; 0x1f4

18              delay.delay_ms(half_period);
   0x08000262 <+90>:    bl      0x8002f5c <stm32f3xx_hal::delay::{{impl}}::delay_ms>
```

该问题的一个解决方案是将`half_period`包装为`Volatile`如下所示。

``` console
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile::Volatile;
use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut half_period = 500_u16;
    let v_half_period = Volatile::new(&mut half_period);

    loop {
        leds[0].on().ok();
        delay.delay_ms(v_half_period.read());

        leds[0].off().ok();
        delay.delay_ms(v_half_period.read());
    }
}

```

编辑`Cargo.toml`文件，在`[dependencies]`地方添加`volatile = "0.4.3"`依赖。

``` console
[dependencies]
aux5 = { path = "auxiliary" }
volatile = "0.4.3"
```

通过使用`Volatile`的上述代码，您现在可以更改`half_period`，并且可以尝试不同的值。
以下是命令列表，后面是解释；`# xxxx`表示演示。

```
$ cargo run --target thumbv7em-none-eabihf   # Compile and load the program into gdb
(gdb) target remote :3333           # Connect to STM32F3DISCOVERY board from PC
(gdb) load                          # Flash program
(gdb) break main.rs:16              # Set breakpoint 1 at top of loop
(gdb) continue                      # Continue, will stop at main.rs:16
(gdb) disable 1                     # Disable breakpoint 1
(gdb) set print asm-demangle on     # Enable asm-demangle
(gdb) disassemble /m                # Disassemble main function
(gdb) continue                      # Led blinking on for 1/2 sec then off 1/2 sec
^C                                  # Stop with Ctrl+C
(gdb) enable 1                      # Enable breakpiont 1
(gdb) continue                      # Continue, will stop at main.rs:16
(gdb) print half_period             # Print half_period result is 500
(gdb) set half_period = 2000        # Set half_period to 2000ms
(gdb) print half_period             # Print half_period and result is 2000
(gdb) disable 1                     # Disable breakpoint 1
(gdb) continue                      # Led blinking on for 2 secs then off 2 sec
^C                                  # Stop with Ctrl+C
(gdb) quit                          # Quit gdb
```

关键的更改出现在源代码的第13、17和20行，您可以在反汇编中看到。在第13行，我们创建`v_half_period`，然后
`read()`第17行和第20行中的值。这意味着，当我们设置`set half_period = 2000`，led现在将打开2秒，然后关闭2秒。

``` console
$ cargo run --target thumbv7em-none-eabihf
   Compiling led-roulette v0.2.0 (~/embedded-discovery/src/05-led-roulette)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `arm-none-eabi-gdb -q ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette`
Reading symbols from ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette...

(gdb) target remote :3333
Remote debugging using :3333
led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:16
16              leds[0].on().ok();

(gdb) load
Loading section .vector_table, size 0x194 lma 0x8000000
Loading section .text, size 0x5258 lma 0x8000194
Loading section .rodata, size 0xbd8 lma 0x80053ec
Start address 0x08000194, load size 24516
Transfer rate: 21 KB/sec, 6129 bytes/write.

(gdb) break main.rs:16
Breakpoint 1 at 0x8000246: file src/05-led-roulette/src/main.rs, line 16.
Note: automatically using hardware breakpoints for read-only addresses.

(gdb) continue
Continuing.

Breakpoint 1, led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:16
16              leds[0].on().ok();

(gdb) disable 1

(gdb) set print asm-demangle on

(gdb) disassemble /m
Dump of assembler code for function _ZN12led_roulette18__cortex_m_rt_main17he1f2bc7990b13731E:
9       fn main() -> ! {
   0x0800020e <+0>:     push    {r7, lr}
   0x08000210 <+2>:     mov     r7, sp
   0x08000212 <+4>:     sub     sp, #72 ; 0x48
   0x08000214 <+6>:     add     r0, sp, #36     ; 0x24

10          let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
   0x08000216 <+8>:     bl      0x800036a <aux5::init>
   0x0800021a <+12>:    b.n     0x800021c <led_roulette::__cortex_m_rt_main+14>
   0x0800021c <+14>:    add     r0, sp, #36     ; 0x24
   0x0800021e <+16>:    add     r1, sp, #8
   0x08000220 <+18>:    ldmia.w r0, {r2, r3, r4, r12, lr}
   0x08000224 <+22>:    stmia.w r1, {r2, r3, r4, r12, lr}
   0x08000228 <+26>:    ldr     r0, [sp, #56]   ; 0x38
   0x0800022a <+28>:    ldr     r1, [sp, #60]   ; 0x3c
   0x0800022c <+30>:    str     r1, [sp, #32]
   0x0800022e <+32>:    str     r0, [sp, #28]
   0x08000230 <+34>:    mov.w   r0, #500        ; 0x1f4

11
12          let mut half_period = 500_u16;
   0x08000234 <+38>:    strh.w  r0, [r7, #-6]
   0x08000238 <+42>:    subs    r0, r7, #6

13          let v_half_period = Volatile::new(&mut half_period);
   0x0800023a <+44>:    bl      0x800033e <volatile::Volatile<&mut u16, volatile::access::ReadWrite>::new<&mut u16>>
   0x0800023e <+48>:    str     r0, [sp, #68]   ; 0x44
   0x08000240 <+50>:    b.n     0x8000242 <led_roulette::__cortex_m_rt_main+52>

14
15          loop {
   0x08000242 <+52>:    b.n     0x8000244 <led_roulette::__cortex_m_rt_main+54>
   0x08000244 <+54>:    add     r0, sp, #28
   0x08000288 <+122>:   b.n     0x8000244 <led_roulette::__cortex_m_rt_main+54>

16              leds[0].on().ok();
=> 0x08000246 <+56>:    bl      0x800032c <switch_hal::output::{{impl}}::on<stm32f3xx_hal::gpio::gpioe::PEx<stm32f3xx_hal::gpio::Output<stm32f3xx_hal::gpio::PushPull>>>>
   0x0800024a <+60>:    b.n     0x800024c <led_roulette::__cortex_m_rt_main+62>
   0x0800024c <+62>:    bl      0x80005fc <core::result::Result<(), core::convert::Infallible>::ok<(),core::convert::Infallible>>
   0x08000250 <+66>:    b.n     0x8000252 <led_roulette::__cortex_m_rt_main+68>
   0x08000252 <+68>:    add     r0, sp, #68     ; 0x44

17              delay.delay_ms(v_half_period.read());
   0x08000254 <+70>:    bl      0x800034a <volatile::Volatile<&mut u16, volatile::access::ReadWrite>::read<&mut u16,u16,volatile::access::ReadWrite>>
   0x08000258 <+74>:    str     r0, [sp, #4]
   0x0800025a <+76>:    b.n     0x800025c <led_roulette::__cortex_m_rt_main+78>
   0x0800025c <+78>:    add     r0, sp, #8
   0x0800025e <+80>:    ldr     r1, [sp, #4]
   0x08000260 <+82>:    bl      0x8002fc4 <stm32f3xx_hal::delay::{{impl}}::delay_ms>
   0x08000264 <+86>:    b.n     0x8000266 <led_roulette::__cortex_m_rt_main+88>
   0x08000266 <+88>:    add     r0, sp, #28

18
19              leds[0].off().ok();
   0x08000268 <+90>:    bl      0x8000882 <switch_hal::output::{{impl}}::off<stm32f3xx_hal::gpio::gpioe::PEx<stm32f3xx_hal::gpio::Output<stm32f3xx_hal::gpio::PushPull>>>>
   0x0800026c <+94>:    b.n     0x800026e <led_roulette::__cortex_m_rt_main+96>
   0x0800026e <+96>:    bl      0x80005fc <core::result::Result<(), core::convert::Infallible>::ok<(),core::convert::Infallible>>
   0x08000272 <+100>:   b.n     0x8000274 <led_roulette::__cortex_m_rt_main+102>
   0x08000274 <+102>:   add     r0, sp, #68     ; 0x44

20              delay.delay_ms(v_half_period.read());
   0x08000276 <+104>:   bl      0x800034a <volatile::Volatile<&mut u16, volatile::access::ReadWrite>::read<&mut u16,u16,volatile::access::ReadWrite>>
   0x0800027a <+108>:   str     r0, [sp, #0]
   0x0800027c <+110>:   b.n     0x800027e <led_roulette::__cortex_m_rt_main+112>
   0x0800027e <+112>:   add     r0, sp, #8
   0x08000280 <+114>:   ldr     r1, [sp, #0]
   0x08000282 <+116>:   bl      0x8002fc4 <stm32f3xx_hal::delay::{{impl}}::delay_ms>
   0x08000286 <+120>:   b.n     0x8000288 <led_roulette::__cortex_m_rt_main+122>

End of assembler dump.

(gdb) continue
Continuing.
^C
Program received signal SIGINT, Interrupt.
0x080037b2 in core::cell::UnsafeCell<u32>::get<u32> (self=0x20009fa0) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1716
1716        }

(gdb) enable 1

(gdb) continue
Continuing.

Breakpoint 1, led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:16
16              leds[0].on().ok();

(gdb) print half_period
$2 = 500

(gdb) disable 1

(gdb) continue
Continuing.
^C
Program received signal SIGINT, Interrupt.
0x08003498 in core::ptr::read_volatile<u32> (src=0xe000e010) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1052
1052        unsafe { intrinsics::volatile_load(src) }

(gdb) enable 1

(gdb) continue
Continuing.

Breakpoint 1, led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:16
16              leds[0].on().ok();

(gdb) print half_period
$3 = 500

(gdb) set half_period = 2000

(gdb) print half_period
$4 = 2000

(gdb) disable 1

(gdb) continue
Continuing.
^C
Program received signal SIGINT, Interrupt.
0x0800348e in core::ptr::read_volatile<u32> (src=0xe000e010) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1046
1046    pub unsafe fn read_volatile<T>(src: *const T) -> T {

(gdb) q
Detaching from program: ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette, Remote target
Ending remote debugging.
[Inferior 1 (Remote target) detached]
```

问题! 如果你开始降低`half_period`的值会发生什么？在`half_period`的值是多少时，你再也看不到LED闪烁了吗？

现在，轮到你编写程序了。
