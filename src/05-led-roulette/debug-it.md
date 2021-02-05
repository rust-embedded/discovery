# Debug it

We are already inside a debugging session so let's debug our program.

After the `load` command, our program is stopped at its *entry point*. This is indicated by the
"Start address 0x8000XXX" part of GDB's output. The entry point is the part of a program that a
processor / CPU will execute first.

The starter project I've provided to you has some extra code that runs *before* the `main` function.
At this time, we are not interested in that "pre-main" part so let's skip right to the beginning of
the `main` function. We'll do that using a breakpoint:

```
(gdb) break main
Breakpoint 1 at 0x80001f0: file src/05-led-roulette/src/main.rs, line 7.
Note: automatically using hardware breakpoints for read-only addresses.

(gdb) continue
Continuing.

Breakpoint 1, led_roulette::__cortex_m_rt_main_trampoline () at src/05-led-roulette/src/main.rs:7
7       #[entry]

(gdb) step
led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:10
10          let x = 42;
```

Breakpoints can be used to stop the normal flow of a program. The `continue` command will let the
program run freely *until* it reaches a breakpoint. In this case, until it reaches `#[entry]`
which is a trampoline to to the main function and where `break main` set the breakpoint.

Note that GDB output says "Breakpoint 1". Remember that our processor can only use six of these
breakpoints so it's a good idea to pay attention to these messages.


OK. Since we are stopped at `#[entry]`. We can advance the program statement by statement using
the `step` command. So let's use that twice times to reach the `_y = x` statement. Once you've typed `step`
once you can just hit enter to run it again, but below we type step twice.

```
(gdb) step
led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:10
10          let x = 42;
(gdb) step
11          _y = x;
```

If you are not using the TUI mode, on each `step` call GDB will print back the current statement
along with its line number.

We are now "on" the `_y = x` statement; that statement hasn't been executed yet. This means that `x`
is initialized but `_y` is not. Let's inspect those stack/local variables using the `print` command:

```
(gdb) print x
$1 = 42
(gdb) print &x
$2 = (*mut i32) 0x20009fe0
(gdb) print _y
$3 = 536870912
(gdb) print &_y
$4 = (*mut i32) 0x20009fe4
```

As expected, `x` contains the value `42`. `_y`, however, contains the value `536870912` (?). Because
`_y` has not been initialized yet, it contains some garbage value.

The command `print &x` prints the address of the variable `x`. The interesting bit here is that GDB
output shows the type of the reference: `*mut i32`, a mutable pointer to an `i32` value. Another interesting
thing is that the addresses of `x` and `_y` are very close to each other: their addresses are just
`4` bytes apart.

Instead of printing the local variables one by one, you can also use the `info locals` command:

```
(gdb) info locals
x = 42
_y = 536870912
```

OK. With another `step`, we'll be on top of the `loop {}` statement:

```
(gdb) step
14          loop {}
```

And `_y` should now be initialized.

```
(gdb) print _y
$5 = 42
```

If we use `step` again on top of the `loop {}` statement, we'll get stuck because the program will
never pass that statement.

> **NOTE** If you used the `step` or any other command by mistake and GDB gets stuck, you can get
it unstuck by hitting `Ctrl+C`.

You can also use the `disassemble /m` command to disassemble the program around the
line you are currently at. You might also want to `set print asm-demangle on`
so the names are demangled, this only needs to be done once a debug session. Later
this and other commands will be placed in an initialization file which will simplify
starting a debug session.

```
(gdb) set print asm-demangle on
(gdb) disassemble /m
Dump of assembler code for function _ZN12led_roulette18__cortex_m_rt_main17h51e7c3daad2af251E:
8       fn main() -> ! {
   0x080001f6 <+0>:     sub     sp, #8
   0x080001f8 <+2>:     movs    r0, #42 ; 0x2a

9           let _y;
10          let x = 42;
   0x080001fa <+4>:     str     r0, [sp, #0]

11          _y = x;
   0x080001fc <+6>:     str     r0, [sp, #4]

12
13          // infinite loop; just so we don't leave this stack frame
14          loop {}
=> 0x080001fe <+8>:     b.n     0x8000200 <led_roulette::__cortex_m_rt_main+10>
   0x08000200 <+10>:    b.n     0x8000200 <led_roulette::__cortex_m_rt_main+10>

End of assembler dump.
```

See the fat arrow `=>` on the left side? It shows the instruction the processor will execute next.

As mentioned above if you were to execute the `step` command GDB gets stuck because it
is executing a branch instruction to itself and never gets past it. So you need to use
`Ctrl+C`. But you can use the `stepi` GDB command, which steps one instruction, and GDB will print
the address **and** line number of the statement the processor will execute next and
it won't get stuck.

```
(gdb) stepi
0x08000194      14          loop {}

(gdb) stepi
0x08000194      14          loop {}
```

One last trick before we move to something more interesting. Enter the following commands into GDB:

```
(gdb) monitor reset halt
Unable to match requested speed 1000 kHz, using 950 kHz
Unable to match requested speed 1000 kHz, using 950 kHz
adapter speed: 950 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08000194 msp: 0x2000a000
(gdb) continue
Continuing.

Breakpoint 1, led_roulette::__cortex_m_rt_main_trampoline () at src/05-led-roulette/src/main.rs:7
7       #[entry]
(gdb) disassemble /m
Dump of assembler code for function main:
7       #[entry]
   0x080001ec <+0>:     push    {r7, lr}
   0x080001ee <+2>:     mov     r7, sp
=> 0x080001f0 <+4>:     bl      0x80001f6 <led_roulette::__cortex_m_rt_main>
   0x080001f4 <+8>:     udf     #254    ; 0xfe

End of assembler dump.
```

We are now back at the beginning of `main`!

`monitor reset halt` will reset the microcontroller and stop it right at the program entry point.
The following `continue` command will let the program run freely until it reaches the `main`
function that has a breakpoint on it.

This combo is handy when you, by mistake, skipped over a part of the program that you were
interested in inspecting. You can easily roll back the state of your program back to its very
beginning.

> **The fine print**: This `reset` command doesn't clear or touch RAM. That memory will retain its
> values from the previous run. That shouldn't be a problem though, unless your program behavior
> depends of the value of *uninitialized* variables but that's the definition of Undefined Behavior
> (UB).

We are done with this debug session. You can end it with the `quit` command.

```
(gdb) quit
A debugging session is active.

        Inferior 1 [Remote target] will be detached.

Quit anyway? (y or n) y
Detaching from program: $PWD/target/thumbv7em-none-eabihf/debug/led-roulette, Remote target
Ending remote debugging.
```

For a nicer debugging experience, you can use GDB's Text User Interface (TUI). To enter into that
mode enter one of the following commands in the GDB shell:

```
(gdb) layout src
(gdb) layout asm
(gdb) layout split
```

> **NOTE** Apologies to Windows users, the GDB shipped with the GNU ARM Embedded Toolchain
> may not support this TUI mode `:-(`.

Below is an example of setting up for a layout split by executing the follow commands:

``` console
$ cargo run --target thumbv7em-none-eabihf
<gdb> target remote :3333
<gdb> load
<gdb> set print asm-demangle on
<gdb> set style sources off
<gdb> break main
<gdb> continue
<gdb> layout split
```

And below the result after `layout split` command is executed:

![GDB session layout split](../assets/gdb-layout-split-1.png "GDB TUI layout split 1")

Now we'll scroll the top source window down so we see the entire file and execute `step`:

![GDB session layout split](../assets/gdb-layout-split-2.png "GDB TUI layout split 2")

Then we'll execute a few `info locals` and `step`'s:

``` console
<gdb> info locals
<gdb> step
<gdb> info locals
<gdb> step
<gdb> info locals
```

![GDB session layout split](../assets/gdb-layout-split-3.png "GDB TUI layout split 3")

At any point you can leave the TUI mode using the following command:

```
(gdb) tui disable
```

![GDB session layout split](../assets/gdb-layout-split-4.png "GDB TUI layout split 4")

> **NOTE** If the default GDB CLI is not to your liking check out [gdb-dashboard]. It uses Python to
> turn the default GDB CLI into a dashboard that shows registers, the source view, the assembly view
> and other things.

[gdb-dashboard]: https://github.com/cyrus-and/gdb-dashboard#gdb-dashboard

Don't close OpenOCD though! We'll use it again and again later on. It's better
just to leave it running. If you want to learn more about what GDB can do, check out the section [How to use GDB](../appendix/2-how-to-use-gdb).


What's next? The high level API I promised.
