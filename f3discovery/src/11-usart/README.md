# USART

<!-- 
The microcontroller has a peripheral called USART, which stands for Universal
Synchronous/Asynchronous Receiver/Transmitter. This peripheral can be configured to work with
several communication protocols like the serial communication protocol.
 -->

使用しているマイクロコントローラは、USART（Universal Synchronous/Asynchronous Receiver/Transmitter）と呼ばれるペリフェラルを持っています。
このペリフェラルは、シリアル通信プロトコルのようないくつかの通信プロトコルで動作するように設定できます。

<!-- 
Throughout this chapter, we'll use serial communication to exchange information between the
microcontroller and your laptop. But before we do that we have to wire up everything.
 -->

この章では、マイクロコントローラとノートPCとの間で情報をやり取りするために、シリアル通信を使います。
しかし、それに取り組む前に、配線する必要があります。

<!-- 
I mentioned before that this protocol involves two data lines: TX and RX. TX stands for transmitter
and RX stands for receiver. Transmitter and receiver are relative terms though; which line is the
transmitter and which line is the receiver depends from which side of the communication you are
looking at the lines.
 -->

前述の通り、このプロトコルは、TXとRXとの2つのデータ線を使います。TXは送信機を、RXは受信機を意味します。
送信機と受信機は、相対的な用語です。つまり、どの線が送信機で、どの線が受信機か、は通信をどちら側から見ているか、に依存します。

<!-- 
We'll be using the pin `PA9` as the microcontroller's TX line and `PA10` as its RX line. In other
words, the pin `PA9` outputs data onto its wire whereas the pin `PA10` listens for data on its
wire.
 -->

`PA9`ピンをマイクロコントローラのTX線として、`PA10`をRX線として、利用します。
言い換えると、`PA9`ピンはワイヤにデータを出力し、`PA10`ピンはワイヤ上のデータを読み取ります。

<!-- 
We could have used a different pair of pins as the TX and RX pins. There's a table in page 44 of the
[Data Sheet] that list all the other possible pins we could have used.
 -->

TXとRXピンとして、別のピンの組み合わせを使うこともできます。[データシート]の44ページの表に、
使用できる他のピンの一覧が記載されています。

<!-- [Data Sheet]: http://www.st.com/resource/en/datasheet/stm32f303vc.pdf -->

[データシート]: http://www.st.com/resource/en/datasheet/stm32f303vc.pdf

<!-- 
The serial module also has TX and RX pins. We'll have to *cross* these pins: that is connect the
microcontroller's TX pin to the serial module's RX pin and the micro's RX pin to the serial module's
TX pin. The wiring diagram below shows all the necessary connections.
 -->

シリアルモジュールにもTXとRXピンがあります。これらのピンを*交差する*必要があります。すなわち、
マイクロコントローラのTXピンをシリアルモジュールのRXピンに、マイクロコントローラのRXピンをシリアルモジュールのTXピンに接続します。
下記の配線図は、必要な全ての接続を示しています。

<p align="center">
<img height=640 title="F3 <-> Serial connection" src="../assets/f3-serial.png">
</p>

<!-- These are the recommended steps to connect the microcontroller and the serial module: -->

マイクロコントローラとシリアルモジュールを接続するためのお勧めの手順は、次の通りです。

<!-- 
- Close OpenOCD and `itmdump`
- Disconnect the USB cables from the F3 and the serial module.
- Connect one of F3 GND pins to the GND pin of the serial module using a female to male (F/M) wire.
  Preferably, a black one.
- Connect the PA9 pin on the back of the F3 to the RXI pin of the serial module using a F/M wire.
- Connect the PA10 pin on the back of the F3 to the TXO pin of the serial module using a F/M wire.
- Now connect the USB cable to the F3.
- Finally connect the USB cable to the Serial module.
- Re-launch OpenOCD and `itmdump`
 -->

- OpenOCDと`itmdump`を終了します。
- USBケーブルをF3とシリアルモジュールから抜きます。
- F3のGNDピンとシリアルモジュールのGNDピンとを、オスメスワイヤを使って接続します。黒色のワイヤが好ましいです。
- F3の背面にあるPA9ピンとシリアルモジュールのRXIピンとを、オスメスワイヤを使って接続します。
- F3の背面にあるPA10ピンとシリアルモジュールのTXOピンとを、オスメスワイヤを使って接続します。
- 次に、USBケーブルとF3とを接続します。
- 最後にUSBケーブルをシリアルモジュールに接続します。
- OpenOCDと`itmdump`を再起動します。

<!-- Everything's wired up! Let's proceed to send data back and forth. -->

配線が全て完了しました！データのやり取りをしましょう。
