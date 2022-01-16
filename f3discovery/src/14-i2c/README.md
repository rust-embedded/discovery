# I2C

<!-- 
We just saw the serial communication protocol. It's a widely used protocol because it's very
simple and this simplicity makes it easy to implement on top of other protocols like Bluetooth and
USB.
 -->

シリアル通信のプロトコルを見てきました。シリアル通信は広く利用されているプロトコルです。
なぜなら、シリアル通信は単純で、単純だからこそBluetoothやUSBのようなプロトコル上に実装することが容易であるためです。

<!-- 
However, it's simplicity is also a downside. More elaborated data exchanges, like reading a digital
sensor, would require the sensor vendor to come up with another protocol on top of it.
 -->

しかし、この単純さは欠点でもあります。例えばデジタルセンサを読み込みようなより複雑なデータのやり取りでは、
センサベンダは別プロトコルの用意を要求します。

<!-- 
(Un)Luckily for us, there are *plenty* of other communication protocols in the embedded space. Some
of them are widely used in digital sensors.
 -->

幸運（不幸）なことに、組込みの世界には*たくさんの*通信プロトコルが存在します。
デジタルセンサで広く使われているものもあります。

<!-- 
The F3 board we are using has three motion sensors in it: an accelerometer, a magnetometer and
gyroscope. The accelerometer and magnetometer are packaged in a single component and can be accessed
via an I2C bus.
 -->

使用しているF3ボードには、加速度計、磁力計、ジャイロスコープの3つのモーションセンサがあります。
加速度計と磁力計は1つのコンポーネントにパッケージされており、I2Cバスでアクセスできます。

<!-- 
I2C stands for Inter-Integrated Circuit and is a *synchronous* *serial* communication protocol. It
uses two lines to exchange data: a data line (SDA) and a clock line (SCL). Because a clock line is
used to synchronize the communication, this is a *synchronous* protocol.
 -->

I2Cは、Inter-Integrated Circuitを意味しており、*同期シリアル*通信プロトコルの1つです。
データをやり取りするために、データ線（SDA）とクロック線（SCL）の2つの線を使用します。
クロック線は通信を同期するために使用するので、これは*同期*プロトコルです。

<p align="center">
<img class="white_bg" height=180 title="I2C bus" src="https://upload.wikimedia.org/wikipedia/commons/3/3e/I2C.svg">
</p>

<!-- 
This protocol uses a *master* *slave* model where the master is the device that *starts* and
drives the communication with a slave device. Several devices, both masters and slaves, can be
connected to the same bus at the same time. A master device can communicate with a specific slave
device by first broadcasting its *address* to the bus. This address can be 7 bits or 10 bits long.
Once a master has *started* a communication with a slave, no other device can make use of the bus
until the master *stops* the communication.
 -->

このプロトコルは、*マスター*・*スレーブ*モデルを使います。ここで、マスターはスレーブデバイスとの通信を*開始*し駆動するデバイスです。
いくつかのデバイス（マスター、スレーブ共に）が同じバスに、同時に接続できます。
マスターデバイスは、まずバスに*アドレス*をブロードキャストすることで、特定のスレーブデバイスと通信できます。
このアドレスは、7ビットか10ビットの長さです。
一度マスターがスレーブと通信を*開始すると*、マスターが通信を*停止する*まで、他のデバイスはバスを使用できません。

<!-- 
The clock line determines how fast data can be exchanged and it usually operates at a frequency of
100 KHz (standard mode) or 400 KHz (fast mode).
 -->

クロック線は、データのやり取りの速度を決定します。通常、100 KHz（standard mode）か400 KHz (fast mode)の周波数で動作します。