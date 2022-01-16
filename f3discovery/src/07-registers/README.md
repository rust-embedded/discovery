<!-- # Registers -->

# レジスタ

<!-- It's time to explore what the `Led` API does under the hood. -->

`Led`のAPIが水面下で何をやっているか、を知る時が来ました。

<!-- 
In a nutshell, it just writes to some special memory regions. Go into the `07-registers` directory
and let's run the starter code statement by statement.
 -->

一言で言えば、いくつかの特別なメモリ領域に書き込みしているだけです。`07-registers`ディレクトリに移動し、スターターコードをステートメントごとに実行しましょう。

``` rust
{{#include src/main.rs}}
```

<!-- What's this magic? -->

この魔法は何でしょうか？

<!-- 
The address `0x48001018` points to a *register*. A register is a special region of memory that
controls a *peripheral*. A peripheral is a piece of electronics that sits right next to the
processor within the microcontroller package and provides the processor with extra functionality.
After all, the processor, on its own, can only do math and logic.
 -->

`0x48001018`番地は、あるレジスタを指しています。レジスタは、メモリの特別な領域で、*ペリフェラル*を制御します。
ペリフェラルは電子部品で、マイクロコントローラパッケージ内のプロセッサのすぐ近くに配置され、プロセッサに追加機能を提供します。
結局のところ、プロセッサ単独では、計算と論理演算しかできません。

<!-- 
This particular register controls General Purpose Input/Output (GPIO) *pins* (GPIO *is* a
peripheral) and can be used to *drive* each of those pins *low* or *high*.
 -->

`0x48001018`番地のレジスタは、汎用入出力（GPIO）*ピン*（GPIO*は*1つのペリフェラルです）を制御します。
そして、そのレジスタは、各ピンを*low*か*high*に駆動するために使用できます。

<!-- ## An aside: LEDs, digital outputs and voltage levels -->

## 余談：LED、デジタル出力と電圧レベル

<!-- Drive? Pin? Low? High? -->

駆動？ピン？Low？High？

<!-- 
A pin is a electrical contact. Our microcontroller has several of them and some of them are
connected to LEDs. An LED, a Light Emitting Diode, will only emit light when voltage is applied to
it with a certain polarity.
 -->

ピンは電気的な接点です。マイクロコントローラは、いくつものピンを持っており、そのうちのいくつかがLEDをに接続されています。
LED（Light Emitting Diode）は、所定の極性で電圧を供給した時のみ、光ります。

<p align="center">
<img class="white_bg" height=180 title="LED circuit" src="https://upload.wikimedia.org/wikipedia/commons/c/c9/LED_circuit.svg">
</p>

<!-- 
Luckily for us, the microcontroller's pins are connected to the LEDs with the right polarity. All
that we have to do is *output* some non-zero voltage through the pin to turn the LED on. The pins
attached to the LEDs are configured as *digital outputs* and can only output two different voltage
levels: "low", 0 Volts, or "high", 3 Volts. A "high" (voltage) level will turn the LED on whereas
a "low" (voltage) level will turn it off.
 -->

幸運なことに、マイクロコントローラのピンは、正しい極性でLEDに接続されています。いくらかの電圧を、ピンを通じて*出力*するだけで、LEDを点灯できます。
LEDに接続されたピンは、*デジタル出力*として設定されており、2つの異なる電圧レベル（「low」が0ボルト、「high」が3ボルト）だけを出力できます。
「high」（電圧）レベルは、LEDを点灯します。一方、「low」（電圧）レベルは、LEDを消灯します。

<!-- 
These "low" and "high" states map directly to the concept of digital logic. "low" is `0` or `false`
and "high" is `1` or `true`. This is why this pin configuration is known as digital output.
 -->

これらの「low」と「high」状態は、デジタル論理の概念に直接結びつきます。「low」は`0`または`false`で
「high」は`1`または`true`です。これが、このピン設定がデジタル出力と呼ばれる理由です。

---

<!-- OK. But how can one find out what this register does? Time to RTRM (Read the Reference Manual)! -->

このレジスタが何をするのか、はどのようにして見つければ良いのでしょうか？リファレンスマニュアルを読む時（RTRM; Read the Reference Manual）が来ました！
