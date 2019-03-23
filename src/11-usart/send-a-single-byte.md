<!-- # Send a single byte -->

# 1バイト送信

<!-- 
Our first task will be to send a single byte from the microcontroller to the laptop over the serial
connection.
 -->

最初のタスクは、シリアル接続経由で、マイクロコントローラからノートPCに1バイト送信することです。

<!-- 
This time, I'm going to provide you with an already initialized USART peripheral. You'll only have
to work with the registers that are in charge of sending and receiving data.
 -->

ここでは、初期化済みのUSARTペリフェラルを提供します。
データの送受信に関連のあるレジスタの操作だけ、行う必要があります。

<!-- 
Go into the `11-usart` directory and let's run the starter code therein. Make sure that you have
minicom/PuTTY open.
 -->

`11-usart`ディレクトリに移動し、そこにあるスターターコードを実行しましょう。
minicom/PuTTYを開いていることを確認して下さい。

``` rust
{{#include src/main.rs}}
```

<!-- 
This program writes to the `TDR` register. This causes the `USART` peripheral to send one byte of
information through the serial interface.
 -->

このプログラムは`TDR`レジスタに書き込みます。このことにより、`USART`ペリフェラルがシリアルインタフェースを通じて、1バイトの情報を送信します。

<!-- 
On the receiving end, your laptop, you should see show the character `X` appear on minicom/PuTTY's
terminal.
 -->

受信側（ノートPC）では、`X`の文字がminicom/PuTTYの端末に現れているはずです。
