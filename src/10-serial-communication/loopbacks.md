<!-- # Loopbacks -->

# ループバック

<!-- 
We've tested sending data. It's time to test receiving it. Except that there's no other device that
can send us some data ... or is there?
 -->

データ送信をテストしました。次はデータ受信をテストします。こちらにデータを送信してくれる他のデバイスがないことを除けば…
それとも、ありますか？

<!-- Enter: loopbacks -->

ループバックを入力します。

<p align="center">
<img title="Serial module loopback" src="../assets/serial-loopback.png">
</p>

<!-- You can send data to yourself! Not very useful in production but very useful for debugging. -->

データを自分自身に送ることができます！製品では役に立ちませんが、デバッグには便利です。

<!-- 
Connect the `TXO` and the `RXI` pins of the serial module together using a male to male jumper wire
as shown above.
 -->

上図のように、シリアルモジュールの`TXO`ピンと`RXI`ピンを、オスオスジャンパワイヤを用いて接続します。

<!-- Now enter some text into minicom/PuTTY and observe. What happens? -->

では、minicom/PuTTYにテキストを入力して、観察して下さい。何が起こりますか？

<!-- You should see three things: -->

3つのことがわかるはずです。

<!-- 
- As before, the TX (red) LED blinks on each key press.
- But now the RX (green) LED blinks on each key press as well! This indicates that the serial module
  is receiving some data; the one it just sent.
- Finally, on the minicom/PuTTY console, you should see that what you type echoes back to the
  console.
 -->

- 以前と同じように、TX（赤色）LEDがキーを押すごとに点滅します。
- しかし今回は、RX（緑色）LEDも、キーを押すごとに点滅します！
  これは、シリアルモジュールが何らかのデータ（送ったデータ）を受信していることを示しています。
- 最後に、minicom/PuTTYコンソール上に、入力がエコーバックされるのが見えるはずです。

<!-- 
Now that you are familiar with sending and receiving data over serial port using minicom/PuTTY,
let's make your microcontroller and your laptop talk!
 -->

ここまでで、minicom/PuTTYを使って、シリアルポート越しにデータを送受信できるようになりました。
マイクロコントローラとノートPCとでやり取りしてみましょう！
