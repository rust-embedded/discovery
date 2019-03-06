# `panic!`

<!-- The `panic!` macro also sends its output to the ITM! -->

`panic!`マクロも出力をITMに送信します！

<!-- Change the `main` function to look like this: -->

`main`関数を次のように変更して下さい。

``` rust
#[entry]
fn main() -> ! {
    panic!("Hello, world!");
}
```

<!-- 
Let's try this program. But before that let's update `openocd.gdb` to run that `monitor` stuff for
us during GDB startup:
 -->

上のプログラムを試してみましょう。ただ、その前に、`monitor`に関連する処理をGDB起動時に実行するように、`openocd.gdb`を更新しましょう。

``` diff
 target remote :3333
 set print asm-demangle on
 set print pretty on
 load
+monitor tpiu config internal itm.txt uart off 8000000
+monitor itm port 0 on
 break main
 continue
```

<!-- OK, now run it. -->

それでは、実行します。

``` console
$ cargo run
(..)
Breakpoint 1, main () at src/06-hello-world/src/main.rs:10
10          panic!("Hello, world!");

(gdb) next
```

<!-- You'll see some new output in the `itmdump` terminal. -->

`itmdump`端末に新しい出力が見えるでしょう。

``` console
$ # itmdump terminal
(..)
panicked at 'Hello, world!', src/06-hello-world/src/main.rs:10:5
```

<!-- FIXME backtraces appear to be broken? -->

<!-- You won't get a `RUST_BACKTRACE` style backtrace in `itmdump`'s output, *but* -->
<!-- you can get the equivalent inside GDB. You already know the command: -->

<!-- ``` -->
<!-- (gdb) backtrace -->
<!-- #0  __bkpt () at asm/bkpt.s:3 -->
<!-- #1  0x08000224 in cortex_m::asm::bkpt () -->
<!--     at $REGISTRY/cortex-m-0.5.2/src/asm.rs:19 -->
<!-- #2  rust_begin_unwind (info=0x10001f84) at src/06-hello-world/auxiliary/src/lib.rs:31 -->
<!-- #3  0x08002548 in core::panicking::panic_fmt () at libcore/panicking.rs:92 -->
<!-- #4  0x080024d8 in core::panicking::panic () at libcore/panicking.rs:53 -->
<!-- #5  0x08000194 in hello_world::main () at src/06-hello-world/src/main.rs:14 -->
<!-- ``` -->

<!-- Ultimately, `panic!` is just another function call so you can see it leaves behind a trace of -->
<!-- function calls. -->

<!-- 
Another thing you can do is catch the panic *before* it does the logging by
putting a breakpoint on the `rust_begin_unwind` symbol.
 -->

他にも、`rust_begin_unwind`シンボルにブレイクポイントを置くことで、ログ出力する*前*にパニックを捕捉することができます。

```
(gdb) monitor reset halt
(..)
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x080026ba msp: 0x10002000

(gdb) break rust_begin_unwind
Breakpoint 2 at 0x80011d2: file $REGISTRY/panic-itm-0.4.0/src/lib.rs, line 46.

(gdb) continue
Continuing.

Breakpoint 2, rust_begin_unwind (info=0x10001fac) at $REGISTRY/panic-itm-0.4.0/src/lib.rs:46
46          interrupt::disable();
```

<!-- 
You'll notice that nothing got printed on the `itmdump` console this time. If
you resume the program using `continue` then a new line will be printed.
 -->

今回は、`itmdump`コンソールに何も表示されないことに気づくでしょう。
`continue`を使ってプログラムを再開すると、新しい行が表示されます。

<!-- In a later section we'll look into other simpler communication protocols. -->

後のセクションでは、他のより簡単な通信プロトコルを検討します。
