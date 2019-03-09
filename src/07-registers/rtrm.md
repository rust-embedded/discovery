<!-- # RTRM: Reading The Reference Manual -->

# RTRM: リファレンスマニュアルを読む

<!-- 
I mentioned that the microcontroller has several pins. For convenience, these pins are grouped in
*ports* of 16 pins. Each port is named with a letter: Port A, Port B, etc. and the pins within each
port are named with numbers from 0 to 15.
 -->

マイクロコントローラはいくつものピンがあると書きました。利便性のために、これらのピンは16ピンからなる*ポート*でグループ化されています。
各ポートは、ポートA、ポートBなどと呼ばれます。そして、各ポート内のピンは、0から15の数字で識別されます。

<!-- 
The first thing we have to find out is which pin is connected to which LED. This information is in
the STM32F3DISCOVERY [User Manual] (You downloaded a copy, right?). In this particular section:
 -->

まず最初に見つけなければならないことは、どのピンがどのLEDに接続されているか、です。
この情報は、STM32F3DISCOVERY [ユーザーマニュアル]（コピーをダウンロードしましたね？）の次のセクションにあります。

<!-- [User Manual]: http://www.st.com/resource/en/user_manual/dm00063382.pdf -->

[ユーザーマニュアル]: http://www.st.com/resource/en/user_manual/dm00063382.pdf

> Section 6.4 LEDs - Page 18

<!-- The manual says: -->

マニュアルには次のように書いてあります。

<!-- 
- `LD3`, the North LED, is connected to the pin `PE9`. `PE9` is the short form of: Pin 9 on Port E.
- `LD7`, the East LED, is connected to the pin `PE11`.
 -->

- `LD3`（北のLED）は`PE9`ピンに接続されています。`PE9`は、ポートEの9番ピンを省略した呼び方です。
- `LD7`（東のLED）は`PE11`に接続されています。

<!-- 
Up to this point, we know that we want to change the state of the pins PE9 and PE11 to turn the
North/East LEDs on/off. These pins are part of Port E so we'll have to deal with the `GPIOE`
peripheral.
 -->

ここまでで、北/東のLEDをオン/オフするためには、PE9ピンとPE11ピンの状態を変えたいことがわかります。
これらのピンはポートEの一部であり。`GPIOE`ペリフェラルを制御しなければなりません。

<!-- 
Each peripheral has a register *block* associated to it. A register block is a collection of
registers allocated in contiguous memory. The address at which the register block starts is known as
its base address. We need to figure out what's the base address of the `GPIOE` peripheral. That
information is in the following section of the microcontroller [Reference Manual]:
 -->

各ペリフェラルは、関連するレジスタ*ブロック*を持っています。レジスタブロックは、連続したメモリに割り当てられたレジスタの集まりです。
レジスタブロックの開始アドレスは、ベースアドレスと呼ばれます。`GPIOE`ペリフェラルのベースアドレスが何か、を見つけ出す必要があります。
この情報は。マイクロコントローラの[リファレンスマニュアル]の次のセクションにあります。

<!-- [Reference Manual]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf -->

[リファレンスマニュアル]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf

> Section 3.2.2 Memory map and register boundary addresses - Page 51

<!-- The table says that base address of the `GPIOE` register block is `0x4800_1000`. -->

この表では、`GPIOE`レジスタブロックのベースアドレスは`0x4800_1000`であると書いてあります。

<!-- 
Each peripheral also has its own section in the documentation. Each of these sections ends with a
table of the registers that the peripheral's register block contains. For the `GPIO` family of
peripheral, that table is in:
 -->

各ペリフェラルは、ドキュメント内にそのペリフェラルのセクションがあります。これらの各セクションは、ペリフェラルのレジスタブロックが内包するレジスタの表が最後に掲載されています。
ペリフェラルの`GPIO`については、その表は次の場所にあります。

> Section 11.4.12 GPIO register map - Page 243

<!-- 
We are interested in the register that's at an offset of `0x18` from the base address of the `GPIOE`
peripheral. According to the table, that would be the register `BSRR`.
 -->

`GPIOE`ペリフェラルのベースアドレスから`0x18`のオフセットにあるレジスタに関心が有ります。
表によると、そのレジスタは`BSRR`です。

<!-- Now we need to jump to the documentation of that particular register. It's a few pages above in: -->

続いて、下記ページにある`BSRR`レジスタのドキュメントに移ります。

> Section 11.4.7 GPIO port bit set/reset register (GPIOx_BSRR) - Page 240

<!-- Finally! -->

ようやく！

<!-- 
This is the register we were writing to. The documentation says some interesting things. First, this
register is write only ... so let's try reading its value `:-)`.
 -->

このレジスタは、私たちが書き込みをしたレジスタです。ドキュメントは、いくつかの興味深いことを示しています。
第一に、このレジスタは書き込み専用です。試しに値を読んでみましょう`:-)`。

<!-- 
We'll use GDB's `examine` command: `x`.
 -->

GDBの`examine`コマンドである`x`を使います。

```
(gdb) next
16              *(GPIOE_BSRR as *mut u32) = 1 << 9;

(gdb) x 0x48001018
0x48001018:     0x00000000

(gdb) # nextコマンドは、北のLEDを点灯します
(gdb) next
19              *(GPIOE_BSRR as *mut u32) = 1 << 11;

(gdb) x 0x48001018
0x48001018:     0x00000000
```

<!-- Reading the register returns `0`. That matches what the documentation says. -->

レジスタを読み込んだ結果は、`0`です。この結果は、ドキュメントに書かれていることと一致します。

<!-- 
The other thing that the documentation says is that the bits 0 to 15 can be used to *set* the
corresponding pin. That is bit 0 sets the pin 0. Here, *set* means outputting a *high* value on
the pin.
 -->

他のおもしろい点は、ドキュメントに、ビット0から15が関連するピンを*設定*するのに使うことができる、と書いてあることです。
*設定*は、*high*の値をピンに出力することを意味します。

<!-- 
The documentation also says that bits 16 to 31 can be used to *reset* the corresponding pin. In this
case, the bit 16 resets the pin number 0. As you may guess, *reset* means outputting a *low* value
on the pin.
 -->

ドキュメントは、ビット16から31はが関連するピンを*リセット*するために使用できる、とも書いてあります。この場合、ビット16は0番ピンをリセットします。
推測通り、*リセット*は、*low*の値をピンに出力することを意味します。

<!-- Correlating that information with our program, all seems to be in agreement: -->

この情報をプログラムと関連付けると、全てが合致しているようです。

<!-- 
- Writing `1 << 9` (`BS9 = 1`)  to `BSRR`  sets `PE9` *high*. That turns the North LED *on*.

- Writing `1 << 11` (`BS11 = 1`) to `BSRR` sets `PE11` *high*. That turns the East LED *on*.

- Writing `1 << 25` (`BR9 = 1`) to `BSRR` sets `PE9` *low*. That turns the North LED *off*.

- Finally, writing `1 << 27` (`BR11 = 1`) to `BSRR` sets `PE11` *low*. That turns the East LED *off*.
 -->

- `1 << 9` (`BS9 = 1`)を`BSRR`に書き込むことは、`PE9`に*high*を設定します。これは、北のLEDを*点灯*します。

- `1 << 11` (`BS11 = 1`)を`BSRR`に書き込むことは、`PE11`に*high*を設定します。これは、東のLEDのを*点灯*します。

- `1 << 25` (`BR9 = 1`)を`BSRR`に書き込むことは、`PE9`に*low*を設定します。これは、北のLEDをを*消灯*します。

- 最後に、`1 << 27` (`BR11 = 1`)を`BSRR`に書き込むことは、`PE11`に*low*を設定します。これは、東のLEDのを*消灯*します。