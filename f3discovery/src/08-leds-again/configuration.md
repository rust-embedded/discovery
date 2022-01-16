<!-- # Configuration -->

# 設定

<!-- 
After turning on the GPIOE peripheral. The peripheral still needs to be configured. In this case, we
want the pins to be configured as digital *outputs* so they can drive the LEDs; by default, most
pins are configured as digital *inputs*.
 -->

GPIOEペリフェラルに電源を入れた後も、GPIOEペリフェラルには、まだ設定が必要です。
今回の場合、LEDを駆動できるように、そのピンをデジタル*出力*として設定したいです。
デフォルトでは、ピンはデジタル*入力*として設定されます。

<!-- You can find the list of registers in the `GPIOE` register block in: -->

`GPIOE`レジスタブロックのレジスタリストは、下記にあります。

> Section 11.4.12 - GPIO registers - Page 243 - Reference Manual

<!-- The register we'll have to deal with is: `MODER`. -->

制御する必要があるレジスタは、`MODER`です。

<!-- 
Your task for this section is to further update the starter code to configure the *right* `GPIOE`
pins as digital outputs. You'll have to:
 -->

このセクションのあなたの仕事は、`GPIOE`ピンをデジタル出力として*正しく*設定するように、スターターコードを更新することです。
次のことに取り組む必要があります。

<!-- 
- Figure out *which* pins you need to configure as digital outputs. (hint: check Section 6.4 LEDs of
  the *User Manual* (page 18)).
- Read the documentation to understand what the bits in the `MODER` register do.
- Modify the `MODER` register to configure the pins as digital outputs.
 -->

- *どの*ピンをデジタル出力に設定しなければならないか、調べて下さい。（ヒント：*ユーザーマニュアル*18ページのSection 6.4 LEDsを見て下さい）
- `MODER`レジスタに含まれるビットが何をするか理解するために、ドキュメントを読んで下さい。
- ピンをデジタル出力に設定するために、`MODER`レジスタを修正して下さい。

<!-- If successful, you'll see the 8 LEDs turn on when you run the program. -->

うまくいくと、プログラム実行時に、8個のLEDが点灯します。
