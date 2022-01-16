<!-- # Debug it -->

# デバッグ

<!-- We are already inside a debugging session so let's debug our program. -->

既にデバッグセッションの中に居ます。プログラムをデバッグしてみましょう。

<!-- 
After the `load` command, our program is stopped at its *entry point*. This is indicated by the
"Start address 0x8000XXX" part of GDB's output. The entry point is the part of a program that a
processor / CPU will execute first.
 -->

`load`コマンドの後、プログラムは、*エントリポイント*で停止しています。このことは、GDB出力の「Start address 0x8000XXX」という部分からわかります。
エントリポイントは、プロセッサ / CPUが最初に実行するプログラムの一部です。

<!-- 
The starter project I've provided to you has some extra code that runs *before* the `main` function.
At this time, we are not interested in that "pre-main" part so let's skip right to the beginning of
the `main` function. We'll do that using a breakpoint:
 -->

スタータープロジェクトでは、`main`関数の*前に*実行する追加のコードを提供しています。
ここでは、「mainの前の」部分には興味がないので、`main`関数の直前までスキップします。ブレイクポイントを使って、これができます。

```
(gdb) break main
Breakpoint 1 at 0x800018c: file src/05-led-roulette/src/main.rs, line 10.

(gdb) continue
Continuing.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, main () at src/05-led-roulette/src/main.rs:10
10          let x = 42;
```

<!-- 
Breakpoints can be used to stop the normal flow of a program. The `continue` command will let the
program run freely *until* it reaches a breakpoint. In this case, until it reaches the `main`
function because there's a breakpoint there.
 -->

ブレイクポイントは、プログラムの通常フローを停止するために使います。`continue`コマンドは、ブレイクポイントに到達する*まで*プログラムを実行します。
今回の場合、`main`関数に到達するまでです。なぜなら、そこにブレイクポイントがあるからです。

<!-- 
Note that GDB output says "Breakpoint 1". Remember that our processor can only use six of these
breakpoints so it's a good idea to pay attention to these messages.
 -->

GDBが「Breakpoint 1」と出力していることに留意して下さい。今回のプロセッサでは、6個のブレイクポイントしか使えないことを思い出して下さい。
これらのメッセージに注意を払うことは大事なことです。

<!-- 
For a nicer debugging experience, we'll be using GDB's Text User Interface (TUI). To enter into that
mode, on the GDB shell enter the following command:
 -->

より良いデバッグ経験のために、GDBのテキストユーザーインタフェース（TUI）を使います。
このモードに入るには、次のコマンドをGDBシェルに入力します。

```
(gdb) layout src
```

<!-- 
> **NOTE** Apologies Windows users. The GDB shipped with the GNU ARM Embedded Toolchain doesn't
> support this TUI mode `:-(`.
 -->

> **注記** Windowsユーザーの方はごめんなさい。GNU ARM Embedded Toolchainで配布されているGDBはTUIモードをサポートしていません`:-(`。

![GDB session](../assets/gdb-layout-src.png "GDB TUI")

<!-- At any point you can leave the TUI mode using the following command: -->

次のコマンドで、いつでもTUIモードから抜けることができます。

```
(gdb) tui disable
```

<!-- 
OK. We are now at the beginning of `main`. We can advance the program statement by statement using
the `step` command. So let's use that twice to reach the `_y = x` statement. Once you've typed `step`
once you can just hit enter to run it
again.
 -->

今、`main`の最初に居ます。`step`コマンドを使って、プログラムをステートメントごとに実行することができます。
`_y = x`ステートメントに到達するために、コマンドを2回使います。一度、`step`を入力すると、エンターを押すだけで、同じコマンドを再び実行できます。

```
(gdb) step
14           _y = x;
```

<!-- 
If you are not using the TUI mode, on each `step` call GDB will print back the current statement
along with its line number.
 -->

TUIモードを使っていない場合、各`step`を呼ぶごとに、GDBは現在のステートメントを行番号と一緒に出力します。

<!-- 
We are now "on" the `_y = x` statement; that statement hasn't been executed yet. This means that `x`
is initialized but `_y` is not. Let's inspect those stack/local variables using the `print` command:
 -->

現在、`_y = x`ステートメントの「上」に居て、まだこのステートメントは実行されていません。
つまり、`x`は初期化されていますが、`_y`は初期化されていません。`print`コマンドを使って、スタック/ローカルな変数を調べてみましょう。

```
(gdb) print x
$1 = 42

(gdb) print &x
$2 = (i32 *) 0x10001ff4

(gdb) print _y
$3 = -536810104

(gdb) print &_y
$4 = (i32 *) 0x10001ff0
```

<!-- 
As expected, `x` contains the value `42`. `_y`, however, contains the value `-536810104` (?). Because
`_y` has not been initialized yet, it contains some garbage value.
 -->

予想通り、`x`は`42`という値を格納しています。しかし、`_y`は、`-536810104` (?)という値を格納しています。
`_y`は、まだ初期化されていないため、ゴミが入っています。

<!-- 
The command `print &x` prints the address of the variable `x`. The interesting bit here is that GDB
output shows the type of the reference: `i32*`, a pointer to an `i32` value. Another interesting
thing is that the addresses of `x` and `_y` are very close to each other: their addresses are just
`4` bytes apart.
 -->

`print &x`コマンドは、変数`x`のアドレスを出力します。ここで興味深いことは、GDBが参照の型を出力することです。`i32*`は、`i32`のポインタ値です。
他におもしろい点は、`x`と`_y`のアドレスがお互いに非常に近いことです。これらのアドレスは、ちょうど`4`バイトだけ離れています。

<!-- Instead of printing the local variables one by one, you can also use the `info locals` command: -->

ローカル変数を1つずつ出力する代わりに、`info locals`コマンドを使うこともできます。

```
(gdb) info locals
x = 42
_y = -536810104
```

<!-- OK. With another `step`, we'll be on top of the `loop {}` statement: -->

では、もう一度`step`を実行すると、`loop {}`ステートメントに到達します。

```
(gdb) step
17          loop {}
```

<!-- And `_y` should now be initialized. -->

そして、`_y`は初期化されているはずです。

```
(gdb) print _y
$5 = 42
```

<!-- 
If we use `step` again on top of the `loop {}` statement, we'll get stuck because the program will
never pass that statement. Instead, we'll switch to the disassemble view with the `layout asm`
command and advance one instruction at a time using `stepi`. You can always switch back into Rust
source code view later by issuing the `layout src` command again.
 -->

`loop {}`ステートメント上で再度`step`を使うと、プログラムがそのステートメントを抜けることがないため、動けなくなります。
代わりに、`layout asm`コマンドで逆アセンブル画面に切り替えます。その上で、`stepi`を使って、1命令ずつ前に進めます。
`layout src`コマンドを使うことで、いつでもRustソースコード画面に戻ることができます。

<!-- > **NOTE** If you used the `step` command by mistake and GDB got stuck, you can get unstuck by hitting `Ctrl+C`. -->

> **注記** 間違って`step`を使ってしまい、GDBが動かなくなった場合、`Ctrl+C`を打つことで、動けるようになります。

```
(gdb) layout asm
```

![GDB session](../assets/gdb-layout-asm.png "GDB disassemble")

<!-- 
If you are not using the TUI mode, you can use the `disassemble /m` command to disassemble the
program around the line you are currently at.
 -->

TUIモードを使っていない場合、`disassemble /m`コマンドを使うことで、現在実行中の行周辺のプログラムを逆アセンブルできます。

```
(gdb) disassemble /m
Dump of assembler code for function main:
7       #[entry]
   0x08000188 <+0>:     sub     sp, #8
   0x0800018a <+2>:     movs    r0, #42 ; 0x2a

8       fn main() -> ! {
9           let _y;
10          let x = 42;
   0x0800018c <+4>:     str     r0, [sp, #4]

11          _y = x;
   0x0800018e <+6>:     ldr     r0, [sp, #4]
   0x08000190 <+8>:     str     r0, [sp, #0]

12
13          // 無限ループ。このスタックフレームから抜けないためのものです。
14          loop {}
=> 0x08000192 <+10>:    b.n     0x8000194 <main+12>
   0x08000194 <+12>:    b.n     0x8000194 <main+12>

End of assembler dump.
```

<!-- See the fat arrow `=>` on the left side? It shows the instruction the processor will execute next. -->

左側にある太矢印`=>`が見えますか？これは、プロセッサが次に実行する命令を示しています。

<!-- 
If not inside the TUI mode on each `stepi` command GDB will print the statement, the line number
*and* the address of the instruction the processor will execute next.
 -->

TUIモードではない場合、各`stepi`コマンドにより、GDBはステートメントと、行番号、*および*プロセッサが次に実行する命令のアドレスを表示します。

```
(gdb) stepi
0x08000194      14          loop {}

(gdb) stepi
0x08000194      14          loop {}
```

<!-- One last trick before we move to something more interesting. Enter the following commands into GDB: -->

さらに興味深いものに行く前に、最後のトリックがあります。GDBに次のコマンドを入力して下さい。

```
(gdb) monitor reset halt
Unable to match requested speed 1000 kHz, using 950 kHz
Unable to match requested speed 1000 kHz, using 950 kHz
adapter speed: 950 kHz
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x08000196 msp: 0x10002000

(gdb) continue
Continuing.

Breakpoint 1, main () at src/05-led-roulette/src/main.rs:10
10          let x = 42;
```

<!-- We are now back at the beginning of `main`! -->

`main`の最初に戻ってきます！

<!-- 
`monitor reset halt` will reset the microcontroller and stop it right at the program entry point.
The following `continue` command will let the program run freely until it reaches the `main`
function that has a breakpoint on it.
 -->

`monitor reset halt`は、マイクロコントローラをリセットし、プログラムのエントリポイントで停止します。
続く`continue`コマンドは、ブレイクポイントがある`main`関数に到達するまで、プログラムを実行します。

<!-- 
This combo is handy when you, by mistake, skipped over a part of the program that you were
interested in inspecting. You can easily roll back the state of your program back to its very
beginning.
 -->

このコンボは、間違って調査の対象とするプログラムの一部をスキップしてしまったときに便利です。
プログラムの状態を、最初の状態に、簡単にロールバックすることができます。

<!-- 
> **The fine print**: This `reset` command doesn't clear or touch RAM. That memory will retain its
> values from the previous run. That shouldn't be a problem though, unless your program behavior
> depends of the value of *uninitialized* variables but that's the definition of Undefined Behavior
> (UB).
 -->

> **ただし書き**：`reset`コマンドは、RAMをクリアしたり、触ったりしません。メモリは、前回実行した時の値を持ち続けています。
> プログラムが、未定義動作の定義である*初期化されていない*変数の値に依存しない限り、このことは問題になりません。

<!-- We are done with this debug session. You can end it with the `quit` command. -->

このデバッグセッションは完了です。`quit`コマンドでデバッグセッションを終了できます。

```
(gdb) quit
A debugging session is active.

        Inferior 1 [Remote target] will be detached.

Quit anyway? (y or n) y
Detaching from program: $PWD/target/thumbv7em-none-eabihf/debug/led-roulette, Remote target
Ending remote debugging.
```

<!-- 
> **NOTE** If the default GDB CLI is not to your liking check out [gdb-dashboard]. It uses Python to
> turn the default GDB CLI into a dashboard that shows registers, the source view, the assembly view
> and other things.
 -->

> **注記** デフォルトのGDBコマンドラインインタフェースが好みでない場合、[gdb-dashboard]を確認して下さい。
> このツールは、Pythonを使用して、デフォルトのGDBコマンドラインインタフェースを、レジスタやソースコード、アセンブリなどを表示するダッシュボードに変換します。

[gdb-dashboard]: https://github.com/cyrus-and/gdb-dashboard#gdb-dashboard

<!-- 
Don't close OpenOCD though! We'll use it again and again later on. It's better
just to leave it running.
 -->

ただし、OpenOCDは、終了しないで下さい！OpenOCDは、この後繰り返し使用します。
動作したままにしておくほうが良いです。

<!-- What's next? The high level API I promised. -->

次は何でしょうか？約束した高レベルのAPIです。