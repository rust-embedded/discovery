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
directory ~/.gdbinit so that it quits immediately:

``` console
$ cat ~/.gdbinit
define hook-quit
  set confirm off
end
```

OK, now use `cargo run` and it stops at `#[entry]`:

``` console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `arm-none-eabi-gdb -q -x openocd.gdb ~/prgs/rust/tutorial/embedded-discovery/target/thumbv7em-none-eabihf/debug/hello-world`
Reading symbols from ~/prgs/rust/tutorial/embedded-discovery/target/thumbv7em-none-eabihf/debug/hello-world...
panic_itm::panic (info=0x20009fa0) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs:57
57	        atomic::compiler_fence(Ordering::SeqCst);
Loading section .vector_table, size 0x194 lma 0x8000000
Loading section .text, size 0x2198 lma 0x8000194
Loading section .rodata, size 0x5d8 lma 0x800232c
Start address 0x08000194, load size 10500
Transfer rate: 17 KB/sec, 3500 bytes/write.
Breakpoint 1 at 0x80001f0: file src/06-hello-world/src/main.rs, line 8.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, hello_world::__cortex_m_rt_main_trampoline () at src/06-hello-world/src/main.rs:8
8	#[entry]
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
Breakpoint 2 at 0x80010f0: file ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs, line 47.

(gdb) info break
Num     Type           Disp Enb Address    What
1       breakpoint     keep n   0x080001f0 in hello_world::__cortex_m_rt_main_trampoline at src/06-hello-world/src/main.rs:8
        breakpoint already hit 1 time
2       breakpoint     keep y   0x080010f0 in panic_itm::panic 
                                           at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs:47

(gdb) c
Continuing.

Breakpoint 2, panic_itm::panic (info=0x20009fa0) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs:47
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
