<!-- # Serial communication -->

# シリアル通信

<a href="https://en.wikipedia.org/wiki/File:Serial_port.jpg">
<p align="center">
<img height="240" title="Standard serial port connector DE-9" src="https://upload.wikimedia.org/wikipedia/commons/thumb/e/ea/Serial_port.jpg/800px-Serial_port.jpg">
</p>
</a>
<!-- 
<p align="center">
<em>This is what we'll be using. I hope your computer has one!</em>
</p>
 -->

<p align="center">
<em>これが、これから使うものです！あなたのラップトップにも1つ付いていると良いのですが！</em>
</p>

<!-- 
Nah, don't worry. This connector, the DE-9, went out of fashion on PCs quite some time ago; it got
replaced by the Universal Serial Bus (USB). We won't be dealing with the DE-9 connector itself but
with the communication protocol that this cable is/was usually used for.
 -->

冗談です、心配しないで下さい。このDE-9というコネクターは、かなり昔にPCでは時代遅れになり、USB（Universal Serial Bus）に置き換わりました。
DE-9コネクタ自体を扱うことはしませんが、このケーブルで使われている/使われていた通信プロトコルを使います。

<!-- 
So what's this *serial communication*? It's an *asynchronous* communication protocol where two
devices exchange data *serially*, as in one bit at a time, using two data lines (plus a common
ground). The protocol is asynchronous in the sense that neither of the shared lines carries a clock
signal. Instead both parties must agree on how fast data will be sent along the wire *before* the
communication occurs. This protocol allows *duplex* communication as data can be sent from A to B
and from B to A simultaneously.
 -->

*シリアル通信*とは何なのでしょうか？それは、2本のデータ線（と共通のグランド）を使って、
2つのデバイスが1回に1ビットずつ*逐次に*データ交換する*非同期*通信プロトコルです。
このプロトコルは、どちらの共有線もクロック信号を伝送しない、という意味で非同期です。
代わりに、双方の当事者は、通信が行われる*前に*、ワイヤに流れるデータの送信速度について合意する必要があります。
このプロトコルは、*双方向の*通信が可能です。データは、AからBへ、BからAへ、同時に送信することができます。

<!-- 
We'll be using this protocol to exchange data between the microcontroller and your laptop. In
contrast to the ITM protocol we have used before, with the serial communication protocol you can
send data from your laptop to the microcontroller.
 -->

このプロトコルを、マイクロコントローラとノートPCとの間のデータ交換に使用します。
これまでに使ったITMプロトコルと違って、シリアル通信プロトコルでは、ノートPCからマイクロコントローラにデータを送ることができます。

<!-- 
The next practical question you probably want to ask is: How fast can we send data through this
protocol?
 -->

次に尋ねたいと思うであろう実用上の質問は、このプロトコルではどのくらいの速度でデータを送信できるのか？ということでしょう。

<!-- 
This protocol works with frames. Each frame has one *start* bit, 5 to 9 bits of payload (data) and 1
to 2 *stop bits*. The speed of the protocol is known as *baud rate* and it's quoted in bits per
second (bps). Common baud rates are: 9600, 19200, 38400, 57600 and 115200 bps.
 -->

このプロトコルは、フレームで動作します。各フレームは、1つの*開始*ビット、5から9ビットのペイロード（データ）と、1か2ビットの*終了ビット*を持ちます。
プロトコルの速度は、*ボーレート*と呼ばれており、ビット毎秒（bps）として示されます。
一般的なボーレートは、9600、19200、38400、57600、115200です。

<!-- 
To actually answer the question: With a common configuration of 1 start bit, 8 bits of data, 1
stop bit and a baud rate of 115200 bps one can, in theory, send 11,520 frames per second. Since each
one frame carries a byte of data that results in a data rate of 11.52 KB/s. In practice, the data
rate will probably be lower because of processing times on the slower side of the communication (the
microcontroller).
 -->

実際に質問に答えると、一般的な設定の開始ビット1ビット、データビット8ビット、終了ビット1ビットでボーレートが115200 bpsであれば、
理論上は、毎秒11,520フレーム送信することが可能です。
各1フレームは、1バイトのデータを伝送するため、11.52KB/秒というデータレートになります。実際には、データレートは、より低くなります。
なぜなら、通信しているより遅い側（マイクロコントローラ）の処理時間があるためです。

<!-- 
Today's laptops/PCs don't support the serial communication protocol. So you can't directly connect
your laptop to the microcontroller. But that's where the serial module comes in. This module will
sit between the two and expose a serial interface to the microcontroller and an USB interface to
your computer. The microcontroller will see your computer as another serial device and your computer
will see the microcontroller as a virtual serial device.
 -->

現在のノートPC/PCは、シリアル通信のプロトコルをサポートしていません。そのため、ノートPCを直接マイクロコントローラに接続することはできません。
しかし、そこでシリアルモジュールの出番です。このモジュールは、2つの機器の間に入り、マイクロコントローラにシリアルインタフェースを、
ノートPCにUSBインタフェースを用意します。
マイクロコントローラは、ノートPCを別のシリアルデバイスとして見るでしょう。そして、ノートPCは、マイクロコントローラを仮想シリアルデバイスとして見ます。

<!-- 
Now, let's get familiar with the serial module and the serial communication tools that your OS
offers. Pick a route:
 -->

それでは、シリアルモジュールと使用しているOSが提供するシリアル通信ツールについて、詳しく学びましょう。ルートを選んで下さい。

- [\*nix](nix-tooling.md)
- [Windows](windows-tooling.md)

[ASC]: https://en.wikipedia.org/wiki/Asynchronous_serial_communication
