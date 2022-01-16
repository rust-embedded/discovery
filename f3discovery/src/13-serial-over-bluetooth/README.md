<!-- # Serial over Bluetooth -->

# Bluetooth経由のシリアル

<!-- 
Now that we verify that the Bluetooth module works with minicom/PuTTY, let's connect it to the
microcontroller:
 -->

minicom/PuTTYでBluetoothモジュールが動くことを検証します。マイクロコントローラを接続しましょう。

<p align="center">
<img height=640 title="F3 <-> Bluetooth connection" src="../assets/f3-bluetooth.png">
</p>

<!-- Recommended steps to wire this up: -->

ワイヤを接続する推奨手順は、次の通りです。

<!-- 
- Close OpenOCD and `itmdump`.
- Disconnect the F3 from your computer.
- Connect F3's GND pin to the module's GND pin using a female to female (F/F) wire (preferably, a
  black one).
- Connect F3's 5V pin to the module's VCC pin using a F/F wire (preferably, a red one).
- Connect the PA9 (TX) pin on the back of the F3 to the Bluetooth's RXD pin using a F/F wire.
- Connect the PA10 (RX) pin on the back of the F3 to the Bluetooth's TXD pin using a F/F wire.
- Now connect the F3 and your computer using an USB cable.
- Re-launch OpenOCD and `itmdump`.
 -->

- OpenOCDと`itmdump`を終了します。
- F3をノートPCから外します。
- メスメスワイヤを使って、F3のGNDピンをモジュールのGNDピンに接続します。（できれば、黒色の線を使います）
- メスメスワイヤを使って、F3の5VピンをモジュールのVCCピンに接続します。（できれば、赤色の線を使います）
- メスメスワイヤを使って、F3背面のPA9（TX）ピンをBluetoothのRXDピンに接続します。
- メスメスワイヤを使って、F3背面のPA10（RX）ピンをBluetoothのTXDピンに接続します。
- USBケーブルを使って、F3とノートPCを接続します。
- OpenOCDと`itmdump`を再起動します。

<!-- 
And that's it! You should be able to run all the programs you wrote in [section 11] without
modification! Just make sure you open the right serial device / COM port.
 -->

これで全てです！[セクション11]で書いたプログラム全てを、修正することなしに、実行することができます。
正しいシリアルデバイス / COMポートを開いていることを確認して下さい。

<!-- [section 11]: ../11-usart/index.html -->

[セクション11]: ../11-usart/index.html
