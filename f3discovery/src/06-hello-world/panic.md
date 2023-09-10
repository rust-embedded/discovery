# `panic!`

`panic!`宏还将其输出发送到ITM！

将`main`函数更改为如下所示：

``` rust
#[entry]
fn main() -> ! {
    panic!("Hello, world!");
}
```

在运行另一个建议之前，我发现在退出gdb时必须进行确认很不方便。
在主目录中添加以下文件。`~/.gdbinit`使其立即退出：

``` console
$ cat ~/.gdbinit
define hook-quit
  set confirm off
end
```

好的，现在使用`cargo run`，它停在`fn main()`的第一行：

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

我们将使用简短的命令名保存输入，输入`c`然后按`Enter`或`Return`键：
```
(gdb) c
Continuing.
```

如果一切正常，您将在`itmdump`终端中看到一些新的输出。

``` console
$ # itmdump terminal
(..)
panicked at 'Hello, world!', src/06-hello-world/src/main.rs:10:5
```

然后输入`Ctrl-c`，它在运行时中断循环：
``` text
^C
Program received signal SIGINT, Interrupt.
0x0800115c in panic_itm::panic (info=0x20009fa0) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs:57
57	        atomic::compiler_fence(Ordering::SeqCst);
```

最终，`panic!`只是另一个函数调用，因此您可以看到它留下了函数调用的痕迹。
这允许您使用`backtrace`或仅使用`bt`，并查看导致死机的调用堆栈：

``` text
(gdb) bt
#0  panic_itm::panic (info=0x20009fa0) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-itm-0.4.2/src/lib.rs:47
#1  0x080005c2 in core::panicking::panic_fmt () at library/core/src/panicking.rs:92
#2  0x0800055a in core::panicking::panic () at library/core/src/panicking.rs:50
#3  0x08000210 in hello_world::__cortex_m_rt_main () at src/06-hello-world/src/main.rs:10
#4  0x080001f4 in hello_world::__cortex_m_rt_main_trampoline () at src/06-hello-world/src/main.rs:8
```

我们可以做的另一件事是在它进行日志记录*之前*拦截panic。
因此，我们将做几件事；重置到开头，禁用断点1，在`rust_begin_unwind`处设置新断点，列出断点，然后继续：

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

您将注意到这次`itmdump`控制台上没有打印任何内容。如果使用`continue`继续程序，则将打印一行新行。

在后面的一节中，我们将探讨其他更简单的通信协议。

最后，输入`q`命令退出，它立即退出，无需确认：

``` text
(gdb) q
Detaching from program: ~/prgs/rust/tutorial/embedded-discovery/target/thumbv7em-none-eabihf/debug/hello-world, Remote target
Ending remote debugging.
[Inferior 1 (Remote target) detached]
```

作为一个更短的队列，您可以输入`Ctrl-d`，这样就省去了一次按键！

> **注意**：在这种情况下，`(gdb)` 提示符被`(quit)`覆盖

``` text
quit)
Detaching from program: ~/prgs/rust/tutorial/embedded-discovery/target/thumbv7em-none-eabihf/debug/hello-world, Remote target
Ending remote debugging.
[Inferior 1 (Remote target) detached]
```
