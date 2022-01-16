<!-- # *nix tooling -->

# *nixのツール

<!-- 
Connect the serial module to your laptop and let's find out what name the OS assigned to it.
 -->

シリアルモジュールをノートPCに接続し、OSがなんという名前を割り当てたか、を確認します。

<!-- 
> **NOTE** On macs, the USB device will named like this: `/dev/cu.usbserial-*`. You won't
> find it using `dmesg`, instead use `ls -l /dev | grep cu.usb` and adjust the following 
> commands accordingly!
 -->

> **注記** macでは、USBデバイスは`/dev/cu.usbserial-*`といった名前になります。
> `dmesg`を使用してもわからないので、代わりに、`ls -l /dev | grep cu.usb`を使って下さい。
> そして、適宜、以下のコマンドを調整して下さい。

``` console
$ dmesg | grep -i tty
(..)
[  +0.000155] usb 3-2: FTDI USB Serial Device converter now attached to ttyUSB0
```

<!-- But what's this `ttyUSB0` thing? It's a file of course! Everything is a file in *nix: -->

しかし、`ttyUSB0`という物は何なのでしょうか？もちろんファイルです！*nixでは、全てがファイルなのです。

``` console
$ ls -l /dev/ttyUSB0
crw-rw-rw- 1 root uucp 188, 0 Oct 27 00:00 /dev/ttyUSB0
```

<!-- 
> **NOTE** if the permissions above is `crw-rw----`, the udev rules have not been set correctly
> see [udev rules](../03-setup/linux.html#udev-rules)
 -->

> **注記** パーミッションが`crw-rw----`の場合、udevルールが正しく設定できていません。
> [udevルール](../03-setup/linux.html#udev-rules)を参照して下さい

<!-- You can send out data by simply writing to this file: -->

単にこのファイルに書き込むだけで、データを送ることができます。

``` console
$ echo 'Hello, world!' > /dev/ttyUSB0
```

<!-- You should see the TX (red) LED on the serial module blink, just once and very fast! -->

シリアルモジュールのTX（赤色）LEDが点滅するのが見えたはずです。非常に速い速度で、ちょうど1回だけ！

## All revisions: minicom

<!-- 
Dealing with serial devices using `echo` is far from ergonomic. So, we'll use the program `minicom`
to interact with the serial device using the keyboard.
 -->

シリアルデバイスを`echo`を使って扱うことは、やりやすい方法ではありません。
そこで、キーボードを使ってシリアルデバイスと通信するために、`minicom`プログラムを使います。

<!-- 
We must configure `minicom` before we use it. There are quite a few ways to do that but we'll use a
`.minirc.dfl` file in the home directory. Create a file in `~/.minirc.dfl` with the following
contents:
 -->

`minicom`を使う前に、設定が必要です。多くの設定方法がありますが、ホームディレクトリの`.minirc.dfl`ファイルを使います。
`~/.minirc.dfl`ファイルを、下記の内容で作成します。

``` console
$ cat ~/.minirc.dfl
pu baudrate 115200
pu bits 8
pu parity N
pu stopbits 1
pu rtscts No
pu xonxoff No
```

<!-- > **NOTE** Make sure this file ends in a newline! Otherwise, `minicom` will fail to read it. -->

> **注記** ファイルが新しい行で終わるようにして下さい！そうでないと、`minicom`がファイルの読み込みに失敗します。

<!-- 
That file should be straightforward to read (except for the last two lines), but nonetheless let's
go over it line by line:
 -->

このファイルは、（最後の2行を除いて）素直に読めますが、1行ずつ順番に見ていきましょう。

<!-- 
- `pu baudrate 115200`. Sets baud rate to 115200 bps.
- `pu bits 8`. 8 bits per frame.
- `pu parity N`. No parity check.
- `pu stopbits 1`. 1 stop bit.
- `pu rtscts No`. No hardware control flow.
- `pu xonxoff No`. No software control flow.
 -->

- `pu baudrate 115200`. ボーレートを115200 bpsに設定します。
- `pu bits 8`. 1フレーム8ビットです。
- `pu parity N`. パリティチェックなし。
- `pu stopbits 1`. ストップビット1ビット。
- `pu rtscts No`. ハードウェア制御フローなし。
- `pu xonxoff No`. ソフトウェア制御フローなし。

<!-- Once that's in place. We can launch `minicom` -->

設定ファイルの作成を終えると、`minicom`を起動できます。

``` console
$ # NOTE you may need to use a different device here
$ minicom -D /dev/ttyACM0 -b 115200
```

<!-- 
This tells `minicom` to open the serial device at `/dev/ttyUSB0` and set its baud rate to 115200.
A text-based user interface (TUI) will pop out.
 -->

このコマンドは、`minicom`に`/dev/ttyUSB0`のシリアルデバイスをオープンするように伝え、ボーレートを115200に設定します。
テキストベースのユーザーインタフェース（TUI）が現れます。

<p align="center">
<img title="minicom" src="../assets/minicom.png">
</p>

<!-- 
You can now send data using the keyboard! Go ahead and type something. Note that the TUI *won't*
echo back what you type but you'll see TX (red) LED on the serial module blink with each keystroke.
 -->

これからは、キーボードを使ってデータを送信できます。何かを入力してみて下さい。TUIは、あなたの入力をエコーバック*しない*ことに気をつけて下さい。
しかし、キーストロークごとにシリアルモジュールのTX（赤色）LEDが点滅するのが見えるでしょう。

<!-- ## `minicom` commands -->

## `minicom`コマンド

<!-- 
`minicom` exposes commands via keyboard shortcuts. On Linux, the shortcuts start with `Ctrl+A`. On
mac, the shortcuts start with the `Meta` key. Some useful commands below:
 -->

`minicom`は、キーボードショートカットによるコマンドを受け付けます。Linuxでは、ショートカットは`Ctrl+A`で始まります。
macでは、ショートカットは`Meta`キーで始まります。便利なコマンドを下記に示します。

<!-- 
- `Ctrl+A` + `Z`. Minicom Command Summary
- `Ctrl+A` + `C`. Clear the screen
- `Ctrl+A` + `X`. Exit and reset
- `Ctrl+A` + `Q`. Quit with no reset
 -->

- `Ctrl+A` + `Z`. Minicomコマンドのまとめ
- `Ctrl+A` + `C`. スクリーンのクリア
- `Ctrl+A` + `X`. 終了とリセット
- `Ctrl+A` + `Q`. リセットなしの終了

<!-- > **NOTE** mac users: In the above commands, replace `Ctrl+A` with `Meta`. -->

> **注記** macユーザーへ：上記コマンドの、`Ctrl+A`は`Meta`で置き換えて下さい。