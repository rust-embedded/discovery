# NOP

如果在上一节中，您以发布模式编译了程序，并实际查看了反汇编，那么您可能会注意到`delay`函数被优化了，
并且从未从`main`中调用。

LLVM 认为该函数没有做任何值得做的事情，于是删除了它。

有一种方法可以防止LLVM优化`for`循环延迟：添加*不稳定的*汇编指令。
任何指令都可以，但NOP（无操作）在这种情况下是一个特别好的选择，因为它没有副作用。

你的`for`循环延迟将变为：

``` rust
#[inline(never)]
fn delay(_tim6: &tim6::RegisterBlock, ms: u16) {
    const K: u16 = 3; // this value needs to be tweaked
    for _ in 0..(K * ms) {
        aux9::nop()
    }
}
```

在发布模式下编译程序时，LLVM不会编译掉这个时间`delay`：

``` console
$ cargo objdump --bin clocks-and-timers --release -- -d --no-show-raw-insn
clocks-and-timers:      file format ELF32-arm-little

Disassembly of section .text:
clocks_and_timers::delay::h711ce9bd68a6328f:
 8000188:       push    {r4, r5, r7, lr}
 800018a:       movs    r4, #0
 800018c:       adds    r4, #1
 800018e:       uxth    r5, r4
 8000190:       bl      #4666
 8000194:       cmp     r5, #150
 8000196:       blo     #-14 <clocks_and_timers::delay::h711ce9bd68a6328f+0x4>
 8000198:       pop     {r4, r5, r7, pc}
```

现在，测试一下：在调试模式下编译程序并运行它，然后在发布模式下编译并运行它。
他们之间有什么区别？ 你认为差异的主要原因是什么？你能想出一种方法使它们变得同等或至少更相似吗？
