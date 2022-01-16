# NOP

<!-- 
If in the previous section you compiled the program in release mode and actually looked at the
disassembly, you probably noticed that the `delay` function is optimized away and never gets called
from within `main`.
 -->

前のセクションで、プログラムをリリースモードでコンパイルし、逆アセンブルした結果を実際に見ると、
`delay`関数が最適化されて`main`から呼び出されないことに気づくでしょう。

<!-- LLVM decided that the function wasn't doing anything worthwhile and just removed it. -->

LLVMは、delay関数が何も価値のあることをやっていないと判断し、関数を削除しました。

<!-- 
There is a way to prevent LLVM from optimizing the `for` loop delay: add a *volatile* assembly
instruction. Any instruction will do but NOP (No OPeration) is a particular good choice in this case
because it has no side effect.
 -->

LLVMが`for`ループでの遅延を最適化しないようにする方法があります。*volatile*アセンブリ命令を追加します。
どのような命令を追加しても良いのですが、今回の場合は、NOP (No OPeration)が特に良い選択です。NOPは副作用がないためです。

<!-- Your `for` loop delay would become: -->

`for`ループでの遅延は、次のようになるでしょう。

``` rust
#[inline(never)]
fn delay(_tim6: &tim6::RegisterBlock, ms: u16) {
    const K: u16 = 3; // この値は微調整が必要です
    for _ in 0..(K * ms) {
        aux9::nop()
    }
}
```

<!-- 
And this time `delay` won't be compiled away by LLVM when you compile your program in release mode:
 -->

今回は、プログラムをリリースモードでコンパイルしても、`delay`はLLVMによって削除されません。

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

<!-- 
Now, test this: Compile the program in debug mode and run it, then compile the program in release
mode and run it. What's the difference between them? What do you think is the main cause of the
difference? Can you think of a way to make them equivalent or at least more similar again?
 -->

では、次のことを試して下さい。プログラムをデバッグモードでコンパイルし、実行します。その後、リリースモードでプログラムをコンパイルし、実行します。
2つの間で何が違いますか？この違いは何が原因と考えますか？
2つを同じものにするか、もしくは、少なくとも似たような振る舞いにする方法を思いつきますか？