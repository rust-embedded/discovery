# Hello, world!

<!-- 
> **HEADS UP** Several readers have reported that the "solder bridge" SB10 (see back of the board)
> on the STM32F3DISCOVERY, which is required to use the ITM and the `iprint!` macros shown below, is
> **not** soldered even though the [User Manual][] (page 21) says that it **should be**.
 -->

> **注意** [ユーザーマニュアル][]（page 21）ではんだ付け**しなければならない**と書いてあるにも関わらず、
> STM32F3DISCOVERY上のSB10「はんだブリッジ」（ボードの裏を見て下さい）がはんだ付け**されていない**、と複数の読者が報告しています。
> これは、後ほど出てくるITMと`iprint!`マクロを使うために必要です。

<!-- 
> **TL;DR** You have two options to fix this: Either **solder** the solder bridge SB10 or connect a
> wire between SWO and PB3 as shown in the picture below.
 -->

> **TL;DR** 2つの選択肢があります。SB10はんだブリッジを**はんだ付けする**か、下記写真の通りSW0とPB3の間をワイヤで接続するか、です。

<!-- [User Manual]: http://www.st.com/resource/en/user_manual/dm00063382.pdf -->

[ユーザーマニュアル]: http://www.st.com/resource/en/user_manual/dm00063382.pdf

<p align="center">
<img height=640 title="Manual SWD connection" src="../assets/f3-swd.png">
</p>

---

<!-- Just a little more of helpful magic before we start doing low level stuff. -->

低レベルのことを始める前に、もう少しだけ役立つ魔法を学んで下さい。

<!-- Blinking an LED is like the "Hello, world" of the embedded world. -->

LEDを点滅させることは、組込みの世界の「Hello, world」です。

<!-- 
But in this section, we'll run a proper "Hello, world" program that prints stuff to your laptop
console.
 -->

しかし、このセクションでは、ラップトップのコンソールに出力するちゃんとした「Hello, world」プログラムを実行します。

<!-- 
Go to the `06-hello-world` directory. There's some starter code in it:
 -->

`06-hello-world`ディレクトリに移動して下さい。その中にスターターコードがあります。

``` rust
{{#include src/main.rs}}
```

<!-- 
The `iprintln` macro will format messages and output them to the microcontroller's *ITM*. ITM stands
for Instrumentation Trace Macrocell and it's a communication protocol on top of SWD (Serial Wire
Debug) which can be used to send messages from the microcontroller to the debugging host. This
communication is only *one way*: the debugging host can't send data to the microcontroller.
 -->

`iprintln`マクロは、メッセージを整え、マイクロコントローラの*ITM*に出力します。ITMは、Instrumentation Trace Macrocellの略であり、
SWD（Serial Wire Debug）の上で通信するプロトコルです。これは、マイクロコントローラからデバッグしているホストにメッセージを送るために使います。
この通信は、*一方向*だけです。デバッグしているホストは、マイクロコントローラにデータを送ることができません。

<!-- 
OpenOCD, which is managing the debug session, can receive data sent through this ITM *channel* and
redirect it to a file.
 -->

OpenOCDは、デバッグセッションを管理し、ITM*チャネル*を通して送信されたデータを受信し、ファイルにリダイレクトします。

<!-- 
The ITM protocol works with *frames* (you can think of them as Ethernet frames). Each frame has a
header and a variable length payload. OpenOCD will receive these frames and write them directly to a
file without parsing them. So, if the microntroller sends the string "Hello, world!" using the
`iprintln` macro, OpenOCD's output file won't exactly contain that string.
 -->

ITMプロトコルは、*フレーム*（イーサネットフレートのようなものだと考えて下さい）で動作します。各フレームは、ヘッダと可変長のペイロードを持ちます。
OpenOCDは、フレームを受信し、フレームを解析せずに、直接ファイルに書き込みます。
マイクロコントローラが、`iprintln`マクロを使用して「Hello, world!」という文字列を送信した場合、
OpenOCDの出力ファイルは、その文字列をそのまま含んでいるわけではありません。

<!-- 
To retrieve the original string, OpenOCD's output file will have to be parsed. We'll use the
`itmdump` program to perform the parsing as new data arrives.
 -->

元の文字列を復元するために、OpenOCDの出力ファイルを解析しなければなりません。
届いた新しいデータの解析を行うために、`itmdump`プログラムを使用します。

<!-- 
You should have already installed the `itmdump` program during the [installation chapter].
 -->

既に`itmdump`プログラムを[インストールの章]でインストールしているはずです。

<!-- [installation chapter]: ../03-setup/index.html#itmdump -->

[インストールの章]: ../03-setup/index.html#itmdump

<!-- 
In a new terminal, run this command inside the `/tmp` directory, if you are using a *nix OS, or from
within the `%TEMP%` directory, if you are running Windows. This should be the same directory from
where you are running OpenOCD.
 -->

*nix OSを使っている場合、新しい端末の`/tmp`ディレクトリ下で、Windowsを使っている場合、`%TEMP%`ディレクトリ下で、
次のコマンドを実行して下さい。これはOpenOCDを実行しているのと、同じディレクトリである必要があります。

<!-- 
> **NOTE** It's very important that both `itmdump` and `openocd` are running
from the same directory!
 -->

> **注記** `itmdump`と`openocd`との両方が、同じディレクトリで実行していることが、非常に重要です。

``` console
$ # itmdumpする端末

$ # *nix
$ cd /tmp && touch itm.txt

$ # Windows
$ cd %TEMP% && type nul >> itm.txt

$ # 両方
$ itmdump -F -f itm.txt
```

<!-- 
This command will block as `itmdump` is now watching the `itm.txt` file. Leave this terminal open.
 -->

このコマンドは、`itmdump`が`itm.txt`を監視している間、ブロックします。この端末は開いたままにしておきます。

<!-- Alright. Now, let's build the starter code and flash it into the microcontroller. -->

では、スターターコードをビルドして、マイクロコントローラのFlashに書き込みましょう。

<!-- 
To avoid passing the `--target thumbv7em-none-eabihf` flag to every Cargo invocation we can set a
default target in .cargo/config:
 -->

`--target thumbv7em-none-eabihf`フラグをCargo呼び出しごとに渡さなくて済むように、.cargo/configにデフォルトターゲットを設定できます。

``` diff
 [target.thumbv7em-none-eabihf]
 runner = "arm-none-eabi-gdb -q -x openocd.gdb"
 rustflags = [
   "-C", "link-arg=-Tlink.x",
 ]

+[build]
+target = "thumbv7em-none-eabihf"
```

<!-- Now if `--target` is not specified Cargo will assume that the target is `thumbv7em-none-eabihf`. -->

これで、`--target`が指定されない場合、Cargoは、ターゲットが`thumbv7em-none-eabihf`だと想定しましす。

``` console
$ cargo run
Reading symbols from target/thumbv7em-none-eabihf/debug/hello-world...done.
(..)
Loading section .vector_table, size 0x400 lma 0x8000000
Loading section .text, size 0x27c4 lma 0x8000400
Loading section .rodata, size 0x744 lma 0x8002be0
Start address 0x8002980, load size 13064
Transfer rate: 18 KB/sec, 4354 bytes/write.
Breakpoint 1 at 0x8000402: file src/06-hello-world/src/main.rs, line 10.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, main () at src/06-hello-world/src/main.rs:10
10          let mut itm = aux6::init();
```

<!-- 
Note that there's a `.gdbinit` at the root of the Cargo project. It's pretty similar to the one we
used in the previous section.
 -->

Cargoプロジェクトのルートディレクトリに`.gdbinit`があることに留意して下さい。
これは、前のセクションで使ったものと非常によく似ています。

<!-- 
Before we execute the `iprintln!` statement. We have to instruct OpenOCD to redirect the ITM output
into the same file that `itmdump` is watching.
 -->

`iprintln!`ステートメントを実行する前に、`itmdump`が監視しているファイルと同じファイルに対して、OpenOCDがITM出力をリダイレクトするように指示しなければなりません。

```
(gdb) # ITMをグローバルに有効化し、itm.txtに全ての出力をリダイレクトします
(gdb) monitor tpiu config internal itm.txt uart off 8000000

(gdb) # ITMポート0を有効にします
(gdb) monitor itm port 0 on
```

<!-- All should be ready! Now execute the `iprintln!` statement. -->

全ての準備が整ったはずです！では、`iprintln`ステートメントを実行します。

```
(gdb) next
12          iprintln!(&mut itm.stim[0], "Hello, world!");

(gdb) next
14          loop {}
```

<!-- You should see some output in the `itmdump` terminal: -->

`itmdump`端末に、何らかの出力が見られるはずです。

``` console
$ itmdump -F -f itm.txt
(..)
Hello, world!
```

<!-- Awesome, right? Feel free to use `iprintln` as a logging tool in the coming sections. -->

素晴らしい、そう思いませんか？以降のセクションで`iprintln`をロギングツールとして、自由に活用して下さい。

<!-- Next: That's not all! The `iprint!` macros are not the only thing that uses the ITM. `:-)` -->

次：これで全てではありません！ITMを使うのは、`iprint!`マクロはだけではありません。`:-)`
