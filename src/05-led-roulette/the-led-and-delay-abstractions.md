# The `Led` and `Delay` abstractions

Now, I'm going to introduce two high level abstractions that we'll use to implement the LED roulette
application.

The auxiliary crate, `aux5`, exposes an initialization function called `init`. When called this
function returns two values packed in a tuple: a `Delay` value and a `Leds` value.

`Delay` can be used to block your program for a specified amount of milliseconds.

`Leds` is actually an array of eight `Led`s. Each `Led` represents one of the LEDs on the F3 board,
and exposes two methods: `on` and `off` which can be used to turn the LED on or off, respectively.

Let's try out these two abstractions by modifying the starter code to look like this:

``` rust
#![deny(unsafe_code)]
#![no_std]

extern crate aux5;

use aux5::prelude::*;
use aux5::{Delay, Leds};

fn main() {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period = 500_u16;

    loop {
        leds[0].on();
        delay.delay_ms(half_period);

        leds[0].off();
        delay.delay_ms(half_period);
    }
}
```

Now build it:

``` console
$ cargo build --target thumbv7em-none-eabihf
```

> **NOTE** It's possible to forget to rebuild the program *before* starting a GDB session; this
> omission can lead to very confusing debug sessions. To avoid this problem you can call `cargo run`
> instead of `cargo build`; `cargo run` will build *and* start a debug session ensuring you never
> forget to recompile your program.

Now, we'll repeat the flashing procedure that we did in the previous section:

``` console
$ # this starts a GDB session of the program; no need to specify the path to the binary
$ arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...done.
(gdb) target remote :3333
Remote debugging using :3333
(..)

(gdb) load
Loading section .vector_table, size 0x188 lma 0x8000000
Loading section .text, size 0x3fc6 lma 0x8000188
Loading section .rodata, size 0xa0c lma 0x8004150
Start address 0x8000188, load size 19290
Transfer rate: 19 KB/sec, 4822 bytes/write.

(gdb) break led_roulette::main
Breakpoint 1 at 0x800021e: file src/main.rs, line 10.

(gdb) continue
Continuing.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, led_roulette::main () at src/main.rs:10
10          let (mut delay, mut leds): (Delay, Leds) = aux5::init();
```

OK. Let's step through the code. This time, we'll use the `next` command instead of `step`. The
difference is that the `next` command will step *over* function calls instead of going inside them.

```
(gdb) next
12          let half_period = 500_u16;

(gdb) next
14          loop {

(gdb) next
15              leds[0].on();

(gdb) next
16              delay.delay_ms(half_period);
```

After executing the `leds[0].on()` statement, you should see a red LED, the one pointing North,
turn on.

Let's continue stepping over the program:

```
(gdb) next
18              leds[0].off();

(gdb) next
19              delay.delay_ms(half_period);
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
Program received signal SIGINT, Interrupt.
core::ptr::read_volatile (src=0x10001f44)
    at $RUST_SRC/libcore/ptr.rs:453
453     pub unsafe fn read_volatile<T>(src: *const T) -> T {
(gdb)
```

In my case, the program stopped its execution inside a `read_volatile` function. GDB output shows
some interesting information about that: `core::ptr::read_volatile (src=0x10001f44)`. This means
that the function comes from the `core` crate and that it was called with argument `src =
0x10001f44`.

Just so you know, a more explicit way to show the arguments of a function is to use the `info args`
command:

```
(gdb) info args
src = 0x10001f44
```

Regardless of where your program may have stopped you can always look at the output of the
`backtrace` command (`bt` for short) to learn how it got there:

```
(gdb) backtrace
#0  core::ptr::read_volatile (src=0x10001f44)
    at $RUST_SRC/libcore/ptr.rs:453
#1  0x0800376a in <vcell::VolatileCell<T>>::get (self=0xe000e010)
    at $REGISTRY/vcell-0.1.0/src/lib.rs:43
#2  <volatile_register::RW<T>>::read (self=0xe000e010)
    at $REGISTRY/volatile-register-0.2.0/src/lib.rs:75
#3  cortex_m::peripheral::syst::<impl cortex_m::peripheral::SYST>::has_wrapped (self=0x10001f44)
    at $REGISTRY/cortex-m-0.4.3/src/peripheral/syst.rs:124
#4  0x08002722 in <stm32f30x_hal::delay::Delay as embedded_hal::blocking::delay::DelayUs<u32>>::delay_us (
    self=0x10001f44, us=500000)
    at $REGISTRY/stm32f30x-hal-0.1.2/src/delay.rs:58
#5  0x08002644 in <stm32f30x_hal::delay::Delay as embedded_hal::blocking::delay::DelayMs<u32>>::delay_ms (
    self=0x10001f44, ms=500)
    at $REGISTRY/stm32f30x-hal-0.1.2/src/delay.rs:32
#6  0x08002686 in <stm32f30x_hal::delay::Delay as embedded_hal::blocking::delay::DelayMs<u16>>::delay_ms (
    self=0x10001f44, ms=500)
    at $REGISTRY/stm32f30x-hal-0.1.2/src/delay.rs:38
#7  0x08000260 in led_roulette::main () at src/main.rs:16
```

`backtrace` will print a trace of function calls from the current function down to main.

Back to our topic. To do what we are after, first, we have to return to the `main` function. We can
do that using the `finish` command. This command resumes the program execution and stops it again
right after the program returns from the current function. We'll have to call it several times.

```
(gdb) finish
Run till exit from #0  core::ptr::read_volatile (src=0x10001f44)
    at $RUST_SRC/libcore/ptr.rs:453
cortex_m::peripheral::syst::<impl cortex_m::peripheral::SYST>::has_wrapped (self=0x10001f44)
    at $REGISTRY/cortex-m-0.4.3/src/peripheral/syst.rs:124
124             self.csr.read() & SYST_CSR_COUNTFLAG != 0
Value returned is $1 = 5

(gdb) finish
Run till exit from #0  cortex_m::peripheral::syst::<impl cortex_m::peripheral::SYST>::has_wrapped (self=0x10001f44)
    at $REGISTRY/cortex-m-0.4.3/src/peripheral/syst.rs:124
0x08002722 in <stm32f30x_hal::delay::Delay as embedded_hal::blocking::delay::DelayUs<u32>>::delay_us (
    self=0x10001f44, us=500000)
    at $REGISTRY/stm32f30x-hal-0.1.2/src/delay.rs:58
58              while !self.syst.has_wrapped() {}
Value returned is $2 = false

(..)

(gdb) finish
Run till exit from #0  0x08002686 in <stm32f30x_hal::delay::Delay as embedded_hal::blocking::delay::DelayMs<u16>>::delay_ms (self=0x10001f44, ms=500)
    at $REGISTRY/stm32f30x-hal-0.1.2/src/delay.rs:38
0x08000260 in led_roulette::main () at src/main.rs:16
16              delay.delay_ms(half_period);
```

We are back in `main`. We have a local variable in here: `half_period`

```
(gdb) info locals
half_period = 500
delay = (..)
leds = (..)
```

Now, we are going to modify this variable using the `set` command:

```
(gdb) set half_period = 100

(gdb) print half_period
$1 = 100
```

If you let program run free again using the `continue` command, you should see that the LED will
blink at a much faster rate now!

Question! What happens if you keep lowering the value of `half_period`? At what value of
`half_period` you can no longer see the LED blink?

Now, it's your turn to write a program.
