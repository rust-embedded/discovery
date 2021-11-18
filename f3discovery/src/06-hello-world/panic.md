# `panic!`

The `panic!` macro also sends its output to the ITM!

Change the `main` function to look like this:

``` rust
#[entry]
fn main() -> ! {
    panic!("Hello, world!");
}
```

Before running one other suggestion, I find it inconvenient to have to
confirm when quitting gdb. Add the following file in your home
directory `~/.gdbinit` so that it quits immediately:

``` console
$ cat ~/.gdbinit
define hook-quit
  set confirm off
end
```

OK, now use `cargo run` and it stops at the first line of `fn main()`:

``` console
$ cargo run
   Compiling hello-world v0.2.0 (~/embedded-discovery/src/06-hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `arm-none-eabi-gdb -q -x ../openocd.gdb ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/hello-world`
Reading symbols from ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/hello-world...
hello_world::__cortex_m_rt_main () at ~/embedded-discovery/src/06-hello-world/src/main.rs:10
10          panic!("Hello, world!");
Loading section .vector_table, size 0x194 lma 0x8000000
Loading section .text, size 0x20fc lma 0x8000194
Loading section .rodata, size 0x554 lma 0x8002290
Start address 0x08000194, load size 10212
Transfer rate: 17 KB/sec, 3404 bytes/write.
Breakpoint 1 at 0x80001f0: file ~/embedded-discovery/src/06-hello-world/src/main.rs, line 8.
Note: automatically using hardware breakpoints for read-only addresses.
Breakpoint 2 at 0x8000222: file ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs, line 570.
Breakpoint 3 at 0x800227a: file ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs, line 560.

Breakpoint 1, hello_world::__cortex_m_rt_main_trampoline () at ~/embedded-discovery/src/06-hello-world/src/main.rs:8
8       #[entry]
hello_world::__cortex_m_rt_main () at ~/embedded-discovery/src/06-hello-world/src/main.rs:10
10          panic!("Hello, world!");
(gdb)
```

We'll use short command names to save typing, enter `c` then the `Enter` or `Return` key:
```
(gdb) c
Continuing.
```

If all is well you'll see some new output in the `itmdump` terminal.

``` console
$ # itmdump terminal
(..)
panicked at 'Hello, world!', src/06-hello-world/src/main.rs:10:5
```

Then type `Ctrl-c` which breaks out of a loop in the runtime:
``` text
^C
Program received signal SIGINT, Interrupt.
0x0800115c in panic_itm::panic (info=0x20009fa0) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs:57
57	        atomic::compiler_fence(Ordering::SeqCst);
```

Ultimately, `panic!` is just another function call so you can see it leaves behind
a trace of function calls. This allows you to use `backtrace` or just `bt` and to see
call stack that caused the panic:

``` text
(gdb) bt
#0  panic_itm::panic (info=0x20009fa0) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs:47
#1  0x080005c2 in core::panicking::panic_fmt () at library/core/src/panicking.rs:92
#2  0x0800055a in core::panicking::panic () at library/core/src/panicking.rs:50
#3  0x08000210 in hello_world::__cortex_m_rt_main () at src/06-hello-world/src/main.rs:10
#4  0x080001f4 in hello_world::__cortex_m_rt_main_trampoline () at src/06-hello-world/src/main.rs:8
```

Another thing we can do is catch the panic *before* it does the logging.
So we'll do several things; reset to the beginning, disable breakpoint 1,
set a new breakpoint at `rust_begin_unwind`, list the break points and then continue:

``` text
(gdb) monitor reset halt
Unable to match requested speed 1000 kHz, using 950 kHz
Unable to match requested speed 1000 kHz, using 950 kHz
adapter speed: 950 kHz
target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x08000194 msp: 0x2000a000

(gdb) disable 1

(gdb) break rust_begin_unwind 
Breakpoint 4 at 0x800106c: file ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs, line 47.

(gdb) info break
Num     Type           Disp Enb Address    What
1       breakpoint     keep n   0x080001f0 in hello_world::__cortex_m_rt_main_trampoline 
                                           at ~/prgs/rust/tutorial/embedded-discovery/src/06-hello-world/src/main.rs:8
        breakpoint already hit 1 time
2       breakpoint     keep y   0x08000222 in cortex_m_rt::DefaultHandler_ 
                                           at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:570
3       breakpoint     keep y   0x0800227a in cortex_m_rt::HardFault_ 
                                           at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:560
4       breakpoint     keep y   0x0800106c in panic_itm::panic 
                                           at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs:47

(gdb) c
Continuing.

Breakpoint 4, panic_itm::panic (info=0x20009fa0) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs:47
47          interrupt::disable();
```

You'll notice that nothing got printed on the `itmdump` console this time. If
you resume the program using `continue` then a new line will be printed.

In a later section we'll look into other simpler communication protocols.

Finally, enter the `q` command to quit and it quits immediately without asking for confirmation:

``` text
(gdb) q
Detaching from program: ~/prgs/rust/tutorial/embedded-discovery/target/thumbv7em-none-eabihf/debug/hello-world, Remote target
Ending remote debugging.
[Inferior 1 (Remote target) detached]
```

As an even shorter sequence you can type `Ctrl-d`, which eliminates
one keystroke!

> **NOTE** In this case the `(gdb)` prompt is overwritten with `quit)`

``` text
quit)
Detaching from program: ~/prgs/rust/tutorial/embedded-discovery/target/thumbv7em-none-eabihf/debug/hello-world, Remote target
Ending remote debugging.
[Inferior 1 (Remote target) detached]
```
