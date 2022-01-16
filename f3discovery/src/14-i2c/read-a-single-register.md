<!-- # Read a single register -->

# 1つのレジスタを読む

<!-- Let's put all that theory into practice! -->

理論を実践してみましょう！

<!-- 
Just like with the USART peripheral, I've taken care of initializing everything before you reach
`main` so you'll only have to deal with the following registers:
 -->

USARTペリフェラルと同様に、`main`に到達する前に全てを初期化しています。
下記レジスタを取り扱うだけで済みます。

<!-- 
- `CR2`. Control register 2.
- `ISR`. Interrupt and status register.
- `TXDR`. Transmit data register.
- `RXDR`. Receive data register.
 -->

- `CR2`. コントロールレジスタ2
- `ISR`. 割込み、ステータスレジスタ
- `TXDR`. 送信データレジスタ
- `RXDR`. 受信データレジスタ

<!-- These registers are documented in the following section of the Reference Manual: -->

これらのレジスタは、リファレンスマニュアルの下記セクションに記載されています。

> Section 28.7 I2C registers - Page 868 - Reference Manual

<!-- We'll be using the `I2C1` peripheral in conjunction with pins `PB6` (`SCL`) and `PB7` (`SDA`). -->

`PB6`（`SCL`）と`PB7`（`SDA`）ピンにつながっている`I2C1`ペリフェラルを使います。

<!-- 
You won't have to wire anything this time because the sensor is on the board and it's already
connected to the microcontroller. However, I would recommend that you disconnect the serial /
Bluetooth module from the F3 to make it easier to manipulate. Later on, we'll be moving the board
around quite a bit.
 -->

今回は、何も配線する必要がありません。センサはボード上にあり、既にマイクロコントローラと接続されているからです。
しかし、ボードを動かしやすくするために、シリアル / BluetoothモジュールをF3から外すことをお勧めします。
後ほど、ボードをかなり動かします。

<!-- 
Your task is to write a program that reads the contents of the magnetometer's `IRA_REG_M` register.
This register is read only and always contains the value `0b01001000`.
 -->

あなたのタスクは、磁力計の`IRA_REG_M`レジスタの内容を読み取るプログラムを書くことです。
このレジスタは読み込み専用で、常に`0b01001000`という値が入っています。

<!-- 
The microcontroller will be taking the role of the I2C master and the magnetometer inside the
LSM303DLHC will be the I2C slave.
 -->

マイクロコントローラは、I2Cマスターの役割を果たします。そしてLSM303DLHC内の磁力計は、I2Cスレーブになります。

<!-- Here's the starter code. You'll have to implement the `TODO`s. -->

スターターコードはこちらです。`TODO`の部分を実装しなければなりません。

``` rust
{{#include src/main.rs}}
```

<!-- To give you some extra help, these are the exact bitfields you'll be working with: -->

追加のヒントです。取り扱う正確なビットフィールドを示します。

- `CR2`: `SADD1`, `RD_WRN`, `NBYTES`, `START`, `AUTOEND`
- `ISR`: `TXIS`, `RXNE`, `TC`
- `TXDR`: `TXDATA`
- `RXDR`: `RXDATA`
