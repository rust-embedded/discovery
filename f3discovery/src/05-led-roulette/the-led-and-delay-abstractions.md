# The `Led` and `Delay` abstractions

Now, I'm going to introduce two high level abstractions that we'll use to implement the LED roulette
application.

The auxiliary crate, `aux5`, exposes an initialization function called `init`. When called this
function returns two values packed in a tuple: a `Delay` value and a `LedArray` value.

`Delay` can be used to block your program for a specified amount of milliseconds.

`LedArray` is an array of eight `Led`s. Each `Led` represents one of the LEDs on the F3 board,
and exposes two methods: `on` and `off` which can be used to turn the LED on or off, respectively.

Let's try out these two abstractions by modifying the starter code to look like this:

``` rust
{{#include examples/the-led-and-delay-abstractions.rs}}
```

Now build it:
``` console
cargo build
```

> **NOTE**: It's possible to forget to rebuild the program *before* starting a GDB session; this
> omission can lead to very confusing debug sessions. To avoid this problem you can call just `cargo run`
> instead of `cargo build`. The `cargo run` command will build *and* start a debug
> session ensuring you never forget to recompile your program.

Now we'll run and repeat the flashing procedure as we did in the previous section
but with the new program. I'll let you type in the `cargo run`, *this will get easier shortly*. :)

> **NOTE**: Don't forget to start ```openocd``` (debugger) on a separate terminal. 
> Otherwise `target remote :3333` won't work!

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

OK. Let's step through the code. This time, we'll use the `next` command instead of `step`. The
difference is that the `next` command will step *over* function calls instead of going inside them.
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

After executing the `leds[0].on().ok()` statement, you should see a red LED, the one pointing North,
turn on.

Let's continue stepping over the program:

```
(gdb) next
17              leds[0].off().ok();

(gdb) next
18              delay.delay_ms(half_period);
```

The `delay_ms` call will block the program for half a second but you may not notice because the
`next` command also takes some time to execute. However, after stepping over the `leds[0].off()`
statement you should see the red LED turn off.

You can already guess what this program does. Let it run uninterrupted using the `continue` command.

```
(gdb) continue
Continuing.
```

Now, let's do something more interesting. We are going to modify the behavior of our program using
GDB.

First, let's stop the infinite loop by hitting `Ctrl+C`. You'll probably end up somewhere inside
`Led::on`, `Led::off` or `delay_ms`:

```
^C
Program received signal SIGINT, Interrupt.
0x08003434 in core::ptr::read_volatile<u32> (src=0xe000e010)
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1053
```

In my case, the program stopped its execution inside a `read_volatile` function. GDB output shows
some interesting information about that: `core::ptr::read_volatile (src=0xe000e010)`. This means
that the function comes from the `core` crate and that it was called with argument `src =
0xe000e010`.

Just so you know, a more explicit way to show the arguments of a function is to use the `info args`
command:

```
(gdb) info args
src = 0xe000e010
```

Regardless of where your program may have stopped you can always look at the output of the
`backtrace` command (`bt` for short) to learn how it got there:

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

`backtrace` will print a trace of function calls from the current function down to main.

Back to our topic. To do what we are after, first, we have to return to the `main` function. We can
do that using the `finish` command. This command resumes the program execution and stops it again
right after the program returns from the current function. We'll have to call it several times.

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

We are back in `main`. We have a local variable in here: `half_period`

```
(gdb) print half_period
$3 = 500
```

Now, we are going to modify this variable using the `set` command:

```
(gdb) set half_period = 100

(gdb) print half_period
$5 = 100
```

If you let program run free again using the `continue` command, you **might** see that the LED will
blink at a much faster rate now, but more likely the blink rate didn't change. **What happened?**

Let's stop the program with `Ctrl+C` and then set a break point at `main:14`.
``` console
(gdb) continue
Continuing.
^C
Program received signal SIGINT, Interrupt.
core::cell::UnsafeCell<u32>::get<u32> (self=0x20009fa4)
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1711
1711        pub const fn get(&self) -> *mut T {
```

Then set a break point at `main.rs:14` and `continue`

``` console
(gdb) break main.rs:14
Breakpoint 2 at 0x8000236: file src/05-led-roulette/src/main.rs, line 14.
(gdb) continue
Continuing.

Breakpoint 2, led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:14
14              leds[0].on().ok();
```

Now open your terminal window so it's about 80 lines long an 170 characters wide if possible.
> **NOTE**: If you can't open the terminal that large, no problem you'll just see
> `--Type <RET> for more, q to quit, c to continue without paging--` so just type return
> until you see the `(gdb)` prompt. Then scroll your terminal window to
> see the results.

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

In the above dump the reason the delay didn't change was because the compiler
recognized that half_period didn't change and instead in the two places where
`delay.delay_ms(half_period);` is called we see `mov.w r1, #500`. So changing the
value of `half_period` does nothing!

``` console
   0x08000244 <+60>:    mov.w   r1, #500        ; 0x1f4

15              delay.delay_ms(half_period);
   0x08000248 <+64>:    bl      0x8002f5c <stm32f3xx_hal::delay::{{impl}}::delay_ms>

(..)

   0x0800025e <+86>:    mov.w   r1, #500        ; 0x1f4

18              delay.delay_ms(half_period);
   0x08000262 <+90>:    bl      0x8002f5c <stm32f3xx_hal::delay::{{impl}}::delay_ms>
```

One solution to the problem is to wrap `half_period` in a `Volatile` as shown below.

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

Edit `Cargo.toml` adding `volatile = "0.4.3"` in the `[dependencies]` section.

``` console
[dependencies]
aux5 = { path = "auxiliary" }
volatile = "0.4.3"
```

With the above code using `Volatile` you can now change `half_period` and
you'll be able to experiment with different values. Here is the list of
commands followed by an explanation; `# xxxx` to demonstrate.

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
(gdb) enable 1                      # Enable breakpoint 1
(gdb) continue                      # Continue, will stop at main.rs:16
(gdb) print half_period             # Print half_period result is 500
(gdb) set half_period = 2000        # Set half_period to 2000ms
(gdb) print half_period             # Print half_period and result is 2000
(gdb) disable 1                     # Disable breakpoint 1
(gdb) continue                      # Led blinking on for 2 secs then off 2 sec
^C                                  # Stop with Ctrl+C
(gdb) quit                          # Quit gdb
```

The critical changes are at lines 13, 17 and 20 in the source code which
you can see in the disassembly. At 13 we create `v_half_period` and then
`read()` its value in lines 17 and 20. This means that when we `set half_period = 2000`
the led will now be on for 2 seconds then off for 2 seconds.

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

Question! What happens if you start lowering the value of `half_period`? At what value of
`half_period` you can no longer see the LED blink?

Now, it's your turn to write a program.
