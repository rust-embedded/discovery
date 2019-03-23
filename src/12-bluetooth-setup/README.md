<!-- # Bluetooth setup -->

# Bluetooth設定

<!-- 
It's time to get rid of some wires. Serial communication can not only be emulated on top of the USB
protocol; it can also be emulated on top of the Bluetooth protocol. This serial over Bluetooth
protocol is known as RFCOMM.
 -->

いくつかのワイヤを取り外す時が来ました。シリアル通信は、USBプロトコル上でのみエミュレートされるものではありません。
シリアル通信は、Bluetoothプロトコル上でもエミュレートできます。このBluetoothプロトコル経由のシリアルは、RFCOMMと呼ばれます。

<!-- 
Before we use the Bluetooth module with the microcontroller, let's first interact with it using
minicom/PuTTY.
 -->

マイクロコントローラでBluetoothモジュールを使う前に、minicom/PuTTYを使って最初の通信を行ってみましょう。

<!-- 
The first thing we'll need to do is: turn on the Bluetooth module. We'll have to share some of the
F3 power to it using the following connection:
 -->

まず最初にやらなければならないのは、Bluetoothモジュールの電源を入れることです。
下記の接続を使って、F3の電源の一部を共有しなければなりません。

<p align="center">
<img height=640 title="F3 <-> Bluetooth connection (power only)" src="../assets/f3-bluetooth-power-only.png">
</p>

<!-- The recommend steps to wire this up are: -->

ワイヤを接続する推奨の手順は、以下の通りです。

<!-- 
- Close OpenOCD and `itmdump`
- Disconnect the USB cables from the F3 and the serial module.
- Connect F3's GND pin to the Bluetooth's GND pin using a female to female (F/F) wire. Preferably, a
  black one.
- Connect F3's 5V pin to the Bluetooth's VCC pin using a F/F wire. Preferably, a red one.
- Then, connect the USB cable back to the F3.
- Re-launch OpenOCD and `itmdump`
 -->

- OpenOCDと`itmdump`を終了します。
- F3とシリアルモジュールからUSBケーブルを抜きます。
- メスメスワイヤを使って、F3のGNDピンをBluetoothのGNDピンに接続します。できれば、黒色のワイヤを使います。
- メスメスワイヤを使って、F3の5VピンとBluetoothのVCCピンを接続します。できれば、赤色のワイヤを使います。
- その後、F3に再びUSBケーブルを挿します。
- OpenOCDと`itmdump`を再起動します。

<!-- 
Two LEDs, a blue one and a red one, on the Bluetooth module should start blinking right after you
power on the F3 board.
 -->

Bluetoothモジュール上の青いLEDと赤いLEDが、F3ボードの電源をオンした直後に、点滅し始めるはずです。

<!-- 
Next thing to do is pair your laptop and the Bluetooth module. AFAIK, Windows and mac users can
simply use their OS default Bluetooth manager to do the pairing. The Bluetooth module default pin
is 1234.
 -->

次にやることは、ノートPCとBluetoothモジュールをペアリングすることです。私の知る限りでは、Windowsとmacユーザーは、
OSデフォルトのBluetoothマネージャをペアリングに使うことができます。
Bluetoothモジュールのデフォルトの暗証番号は、1234です。

<!-- Linux users will have to follow (some of) [these instructions]. -->

Linuxユーザーは、[これらの手順]（のいくつかに）従う必要があります。

<!-- [these instructions]: linux.md -->

[これらの手順]: linux.md
