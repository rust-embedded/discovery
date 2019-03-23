<!-- # Loopback, again -->

# ループバック、再び

<!-- 
After pairing your laptop to the Bluetooth module, your OS should have created a device file / COM
port for you. On Linux, it should be `/dev/rfcomm*`; on mac, it should be `/dev/cu.*`; and on
Windows, it should be a new COM port.
 -->

ノートPCをBluetoothモジュールとペアリングした後、OSはデバイスファイルファイル/COMポートを作ってくれているはずです。
Linuxでは`/dev/rfcomm*`で、macでは`/dev/cu.*`で、Windowsでは新しいCOMポートです。

<!-- 
We can now test the Bluetooth module with minicom/PuTTY. Because this module doesn't have LED
indicators for the transmission and reception events like the serial module did, we'll test the
module using a loopback connection:
 -->

これで、minicom/PuTTYでBluetoothモジュールのテストができます。
このモジュールは、シリアルモジュールにあったような送受信イベントのためのLEDインジケータがないため、
ループバック接続を使って、モジュールをテストします。

<p align="center">
<img height=640 title="F3 <-> Bluetooth connection (loopback)" src="../assets/f3-bluetooth-loopback.png">
</p>

<!-- Just connect the module's TXD pin to its RXD pin using a F/F wire. -->

メスメスワイヤを使って、モジュールのTXDピンをRXDピンに接続するだけです。

<!-- Now, connect to the device using `minicom`/`PuTTY`: -->

次に、`minicom`/`PuTTY`を使って、デバイスに接続します。

``` console
$ minicom -D /dev/rfcomm0
```

<!-- 
Upon connecting, the blinking pattern of the Bluetooth module should change to: long pause then
blink twice quickly.
 -->

接続時、Bluetoothモジュールの点滅パターンが変わります。長い消灯の後、素早く2回点滅します。

<!-- Typing inside minicom/PuTTY terminal should echo back what you type. -->

minicom/PuTTY端末の中でタイピングすると、タイピングしたものがエコーバックされるはずです。
