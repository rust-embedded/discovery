# The `led` and `delay` modules

Now, I'm going to introduce two high level modules that we'll use to implement
the LED roulette application.

The Playground crate, `pg`, exposes the `delay` and `led` modules.

The `delay` module exposes a `ms` function that can block your program for `n`
milliseconds.

The `led` modules exposes a `LEDS` static variable that holds 8 `Led` structs
in an array. Each `Led` struct represents an LED on the F3 board and exposes
two methods: `on` and `off` which can be used to turn the LED on or off,
respectively.

Let's try out this API by modifying the starter code to look like this:

``` rust
#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate pg;

use pg::delay;
use pg::led::LEDS;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let half_period = 500; // ms

    loop {
        LEDS[0].on();
        delay::ms(half_period);

        LEDS[0].off();
        delay::ms(half_period);
    }
}
```

Now build it:

```
$ xargo build --target thumbv7em-none-eabihf
```

> **NOTE** It's quite common to forget to rebuild the program *before* starting
> a GDB session. This omission can lead to very confusing debug sessions. Always
> make sure to call `xargo build` **before** calling `gdb`, or reload the
> binary using `file /path/to/elf`

Now, we'll have to repeat the flashing procedure that we did in the previous
section:

```
$ arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...done.
(gdb) target remote :3333
Remote debugging using :3333
(..)

(gdb) load
Loading section .text, size 0x76c8 lma 0x8000000
Loading section .ARM.extab.text._ZN44_$LT$char$u20$as$u20$core..char..CharExt$GT$11encode_utf817h4f3134c02513b5e1E, size 0xc lma 0x80076c8
Loading section .ARM.extab.text._ZN4core3fmt9Formatter11debug_tuple17hf0ed23ebdee33c00E, size 0xc lma 0x80076d4
Start address 0x8000194, load size 30432
Transfer rate: 22 KB/sec, 7608 bytes/write.

(gdb) break main
Breakpoint 1 at 0x80001e6: file $PWD/src/main.rs, line 10.

(gdb) continue
Continuing.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, led_roulette::main () at $PWD/src/main.rs:10
10      pub fn main() -> ! {
```

OK. Let's step through the code. This time, we'll use the `next` command instead
of `step`. The difference is that the `next` command will step *over* function
calls instead of going inside them.

```
(gdb) next
13          let half_period = 500; // ms

(gdb) next
15          loop {

(gdb) next
16              LEDS[0].on();

(gdb) next
17              delay::ms(half_period);
```

After executing the `LEDS[0].on()` statement, you should see a red LED, the one
"pointing North", turn on.

Let's continue stepping over the program:

```
(gdb) next
19              LEDS[0].off();

(gdb) next
20              delay::ms(half_period);
```

The `delay::ms` call will block the program for half a second but you may not
notice because the `next` command also takes some time to execute. However,
after stepping over the `LEDS[0].off()` statement you should see the red LED
turn off.

You can already guess what this program does. Let it run uninterrupted using the
`continue` command.

```
(gdb) continue
Continuing.
```

Now, let's do something more interesting. We are going to modify the behavior of
our program using GDB.

First, let's stop the infinite loop by hitting `Ctrl+C`. You'll probably end up
somewhere inside `Led::on`, `Led::off` or `delay::ms`:

```
Program received signal SIGINT, Interrupt.
0x08000d04 in core::ptr::read_volatile<u32> (src=0x40001410)
    at $SYSROOT/lib/rustlib/src/rust/src/libcore/ptr.rs:213
213     pub unsafe fn read_volatile<T>(src: *const T) -> T {
(gdb)
```

In my case, the program stopped its execution inside a `read_volatile` function.
GDB output shows some interesting information about that:
`core::ptr::read_volatile<u32> (src=0x40001410)`. This means that: the function
comes from the `core` crate, it's originally a generic function but we are
dealing with a `u32` instance of it and that it was called with argument `src =
0x40001410`.

Just so you know, a more explicit way to show the arguments of a function is to
use the `info args` command:

```
(gdb) info args
src = 0x40001410
```

Regardless of where your program may have stopped you can always look at output
of the `backtrace` command (`bt` for short) to learn how you got there:

```
(gdb) backtrace
#0  0x08000d04 in core::ptr::read_volatile<u32> (src=0x40001410)
    at $SYSROOT/lib/rustlib/src/rust/src/libcore/ptr.rs:213
#1  0x08004280 in volatile_register::RW<u32>::read<u32> (self=<optimized out>)
    at $VOLATILE_REGISTER/src/lib.rs:71
#2  f3::peripheral::tim::Sr::read (self=0x40001410)
    at $F3/master/src/peripheral/tim.rs:321
#3  0x080014d6 in f3::delay::ms (n=500) at $F3/master/src/delay.rs:23
#4  0x08000210 in led_roulette::main () at $PWD/src/main.rs:15
```

`backtrace` will print a trace of function calls from the current function down
to main.

Back to our topic. To do what we are after, first, we have to return to the
`main` function. We can do that using the `finish` command. This command resumes
the program execution and stops it again right after the program returns from
the current function. We'll have to call it several times.

```
(gdb) finish
Run till exit from #0  0x08000d04 in core::ptr::read_volatile<u32> (src=0x40001410)
    at $SYSROOT/lib/rustlib/src/rust/src/libcore/ptr.rs:213
f3::peripheral::tim::Sr::read (self=0x40001410)
    at $F3/master/src/peripheral/tim.rs:321
321             SrR { bits: self.register.read() }
Value returned is $1 = 0

(gdb) finish
Run till exit from #0  f3::peripheral::tim::Sr::read (self=0x40001410)
    at $F3/src/peripheral/tim.rs:321
0x080014d6 in f3::delay::ms (n=500) at $F3/src/delay.rs:23
23              while !tim7.sr.read().uif() {}
Value returned is $2 = f3::peripheral::tim::SrR {bits: 0}

(gdb) finish
Run till exit from #0  0x080014d6 in f3::delay::ms (n=500)
    at $F3/src/delay.rs:23
0x08000210 in led_roulette::main () at $PWD/src/main.rs:15
15              delay::ms(half_period);
```

We are back in `main`. We have a local variable in here: `half_period`

```
(gdb) info locals
half_period = 500
```

Now, we are going to modify this variable using the `set` command:

```
(gdb) set half_period = 100

(gdb) print half_period
$1 = 100
```

If you let program run free again using the `continue` command, you should see
that the LED will blink at a much faster rate now!

Question! What happens if you keep lowering the value of `half_period`? At what
value of `half_period` you can no longer see the LED blink?

Now, it's your turn to write a program.
