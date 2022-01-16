<!-- # Windows tooling -->

<<<<<<< HEAD:src/10-serial-communication/windows-tooling.md
# Windowsのツール

<!-- Before plugging the Serial module, run the following command on the terminal: -->

シリアルモジュールを差し込む前に、ターミナルで次のコマンドを実行して下さい。
=======
Start by unplugging your micro:bit.

Before plugging the micro:bit, run the following command on the terminal:
>>>>>>> upstream/master:microbit/src/06-serial-communication/windows-tooling.md

``` console
$ mode
```

<<<<<<< HEAD:src/10-serial-communication/windows-tooling.md
<!-- 
It will print a list of devices that are connected to your laptop. The ones that start with `COM` in
=======
It will print a list of devices that are connected to your computer. The ones that start with `COM` in
>>>>>>> upstream/master:microbit/src/06-serial-communication/windows-tooling.md
their names are serial devices. This is the kind of device we'll be working with. Take note of all
the `COM` *ports* `mode` outputs *before* plugging the serial module.
 -->

このコマンドは、ノートPCに接続されているデバイスの一覧を表示します。`COM`から名前が始まるデバイスが、シリアルデバイスです。
このデバイスがこれから使うデバイスの種類です。シリアルモジュールを差し込む*前に*`mode`が出力した全ての`COM`*ポート*をメモして下さい。

<<<<<<< HEAD:src/10-serial-communication/windows-tooling.md
<!-- 
Now, plug the Serial module and run the `mode` command again. You should see a new `COM` port appear
on the list. That's the COM port assigned to the serial module.
 -->

それでは、シリアルモジュールを差し込み、`mode`コマンドを再び実行して下さい。新しい`COM`ポートが、リストに現れるはずです。
これが、シリアルモジュールに割り当てらたCOMポートです。

<!-- Now launch `putty`. A GUI will pop out. -->
=======
Now, plug the micro:bit and run the `mode` command again. If you see a new
`COM` port appear on the list then you have that's the COM port assigned to the
serial functionality on the micro:bit.
>>>>>>> upstream/master:microbit/src/06-serial-communication/windows-tooling.md

それでは、`putty`を起動します。GUIが現れます。

<p align="center">
<img title="PuTTY settings" src="../assets/putty-settings.png">
</p>

<!-- 
On the starter screen, which should have the "Session" category open, pick "Serial" as the
"Connection type". On the "Serial line" field enter the `COM` device you got on the previous step,
for example `COM3`.
 -->

開始画面では、「Session」カテゴリがあるはずなので、それを開いて「Connection type」として「Serial」を選択して下さい。
「Serial line」フィールドには、先ほどの手順で入手した`COM`デバイスを入力して下さい。例えば、`COM3`です。

<!-- 
Next, pick the "Connection/Serial" category from the menu on the left. On this new view, make sure
that the serial port is configured as follows:
 -->

次に、メニューの左側から、「Connection/Serial」カテゴリを選択します。新しい画面では、
シリアルポートが次の通り設定されていることを確認して下さい。

- "Speed (baud)": 115200
- "Data bits": 8
- "Stop bits": 1
- "Parity": None
- "Flow control": None

<!-- Finally, click the Open button. A console will show up now: -->

最後に、Openボタンをクリックします。コンソールが出現します。

<p align="center">
<img title="PuTTY console" src="../assets/putty-console.png">
</p>

<<<<<<< HEAD:src/10-serial-communication/windows-tooling.md
<!-- 
If you type on this console, the TX (red) LED on the Serial module should blink. Each key stroke
=======
If you type on this console, the yellow LED on top of the micro:bit will blink. Each keystroke
>>>>>>> upstream/master:microbit/src/06-serial-communication/windows-tooling.md
should make the LED blink once. Note that the console won't echo back what you type so the screen
will remain blank.
 -->

このコンソールでタイピングすると、シリアルモジュールのTX（赤色）LEDが点滅するはずです。
キーストロークごとにLEDは1一度点滅します。コンソールは、タイピングしたことをエコーバックしないため、
画面は何も表示されていないままになります。
