<!-- # General troubleshooting -->

# トラブルシューティング

<!-- ## OpenOCD problems -->

## OpenOCDの問題

<!-- ### can't connect to OpenOCD - "Error: open failed" -->

### OpenOCDに接続できません - "Error: open failed"

<!-- #### Symptoms -->

#### 症状

<!-- 
Upon trying to establish a *new connection* with the device you get an error
that looks like this:
 -->

デバイスと*新しい接続*を確立しようとするときに、次のようなエラーが発生します。

```
$ openocd -f (..)
(..)
Error: open failed
in procedure 'init'
in procedure 'ocd_bouncer'
```

<<<<<<< HEAD:src/appendix/1-general-troubleshooting/README.md
<!-- #### Cause + Fix -->

#### 原因と解決策

<!-- 
- All: The device is not (properly) connected. Check the USB connection using
  `lsusb` or the Device Manager.
- Linux: You may not have enough permission to open the device. Try again with
  `sudo`. If that works, you can use [these instructions] to make OpenOCD work
  without root privilege.
- Windows: You are probably missing the ST-LINK USB driver. Installation
  instructions [here].
 -->

- 全員：デバイスが（正しく）接続されていません。`lsusb`かデバイスマネージャでUSB接続を確認して下さい。
- Linux：デバイスを開くためのパーミッションがないかもしれません。`sudo`をつけてもう1度試して下さい。
  もしこれがうまくいくようであれば、OpenOCDをroot権限なしで実行するために、[これらの手順]が使えます。
- Windows：ST-LINK USBドライバがない可能性があります。インストール手順は[こちら]です。
=======
#### Cause

The device is not (properly) connected or not the correct ST-LINK interface
configuration is used.

#### Fix

Linux:

- Check the USB connection using `lsusb`.
- You may not have enough permission to open the device. Try again with `sudo`.
  If that works, you can use [these instructions] to make OpenOCD work without
  root privilege.
- You might be using the wrong interface configuration for your ST-LINK.
  Try `interface/stlink-v2.cfg` instead of `interface/stlink-v2-1.cfg`.
>>>>>>> upstream/master:f3discovery/src/appendix/1-general-troubleshooting/README.md

<!-- 
[these instructions]: ../../03-setup/linux.md#udev-rules

Windows:

- You are probably missing the ST-LINK USB driver. Installation instructions
  [here].

[here]: ../../03-setup/windows.md#st-link-usb-driver
 -->

[これらの手順]: ../../03-setup/linux.md#udev-rules
[こちら]: ../../03-setup/windows.md#st-link-usb-driver

<!-- ### can't connect to OpenOCD - "Polling again in X00ms" -->

### OpenOCDに接続できません - "Polling again in X00ms"

<!-- #### Symptoms -->

#### 症状

<!-- 
Upon trying to establish a *new connection* with the device you get an error
that looks like this:
 -->

デバイスと*新しい接続*を確立しようとするときに、次のようなエラーが発生します。

```
$ openocd -f (..)
(..)
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Previous state query failed, trying to reconnect
```

<!-- #### Cause -->

#### 原因

<!-- 
The microcontroller may have get stuck in some tight infinite loop or it may be
continuously raising an exception, e.g. the exception handler is raising an
exception.
 -->

マイクロコントローラは、短い無限ループに陥っているか、継続的に例外が発生している可能性があります。
例えば、例外ハンドラが例外を発生させています。

<!-- #### Fix -->

#### 解決策

<!-- 
- Close OpenOCD, if running
- Press and hold the reset (black) button
- Launch the OpenOCD command
- Now, release the reset button
 -->

- もし実行中であれば、OpenOCDを終了します
- （黒い）リセットボタンを押したままにします
- OpenOCDコマンドを実行します
- ここで、リセットボタンを離します

<!-- ### OpenOCD connection lost - "Polling again in X00ms" -->

### OpenOCD接続切れ - "Polling again in X00ms"

<!-- #### Symptoms -->

#### 症状

<!-- A *running* OpenOCD session suddenly errors with: -->

*実行中の*OpenOCDセッションが、急にエラーになります。

```
# openocd -f (..)
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Previous state query failed, trying to reconnect
```

<!-- #### Cause -->

#### 原因

<!-- The USB connection was lost. -->

USB接続が切断されました。

<!-- #### Fix -->

#### 解決方法

<!-- 
- Close OpenOCD
- Disconnect and re-connect the USB cable.
- Re-launch OpenOCD
 -->

- OpenOCDを終了します
- USBケーブルを抜き、再度接続します
- OpenOCDを再起動します

<!-- ### Can't flash the device - "Ignoring packet error, continuing..." -->

### デバイスのFlashに書き込めません - "Ignoring packet error, continuing..."

<!-- #### Symptoms -->

#### 症状

<!-- While flashing the device, you get: -->

デバイスのFlashに書き込んでいる間に、次のエラーが発生します。

```
$ arm-none-eabi-gdb $file
Start address 0x8000194, load size 31588
Transfer rate: 22 KB/sec, 5264 bytes/write.
Ignoring packet error, continuing...
Ignoring packet error, continuing...
```

<!-- #### Cause -->

#### 原因

<!-- 
Closed `itmdump` while a program that "printed" to the ITM was running. The
current GDB session will appear to work normally, just without ITM output but
the next GDB session will error with the message that was shown in the previous
section.
 -->

ITMに「表示する」プログラムが実行している間に`itmdump`を終了したことです。
現在のGDBセッションは、ITM出力なしに、通常通り動きますが、次のGDBセッションは先程お見せしたメッセージと共にエラーになります。

<!-- 
Or, `itmdump` was called **after** the `monitor tpiu` was issued thus making
`itmdump` delete the file / named-pipe that OpenOCD was writing to.
 -->

もしくは、`monitor tpiu`が発行された*後に*`itmdump`を実行したことです。
そのため、`itmdump`はOpenOCDが書き込んでいたファイル / 名前付きのパイプを削除します。

<!-- #### Fix -->

#### 解決策

<!-- 
- Close/kill GDB, OpenOCD and `itmdump`
- Remove the file / named-pipe that `itmdump` was using (for example,
  `itm.txt`).
- Launch OpenOCD
- Then, launch `itmdump`
- Then, launch the GDB session that executes the `monitor tpiu` command.
 -->

- GDB、OpenOCD、`itmdump`を終了/killします。
- `itmdump`が使っていたファイル / 名前付きのパイプ（例えば、`itm.txt`）を削除します。
- OpenOCDを起動します。
- 次に、`itmdump`を起動します。
- 次に、`monitor tpiu`コマンドを実行するGDBセッションを起動します。

<<<<<<< HEAD:src/appendix/1-general-troubleshooting/README.md
<!-- ## Cargo problems -->

## Cargoの問題
=======

### can't connect to OpenOCD - "Error: couldn't bind [telnet] to socket: Address already in use"

#### Symptoms

Upon trying to establish a *new connection* with the device you get an error
that looks something like this:

```
$ openocd -f (..)
(..)
Error: couldn't bind telnet to socket: Address already in use
```

#### Cause

One or more of the ports OpenOCD requires access to, 3333, 4444, or 6666, is in use by another process. Each of these ports is used for another aspect: 3333 for gdb, 4444 for telnet, 6666 for remote procedure call (RPC) commands to TCL

#### Fix

You can go two routes for fixing this. A) Kill any process that's using one of those ports. B) Specify different ports you know to be free for OpenOCD to use.

Solution A

Mac:
- Get a list of processes using ports by running `sudo lsof -PiTCP -sTCP:LISTEN`
- Kill the process(es) blocking the key ports by noting their pid(s) and running `kill [pid]` for each. (Assuming you can confirm they're not running anything mission-critical on your machine!)

Solution B

All:
- Send configuration details to OpenOCD when starting it up so that it uses a different port from the default for any of the processes.
- For example, to do its telnet features on 4441 instead of the default 4444, you would run `openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg -c "telnet_port 4441"`
- More details on OpenOCD's Configuration Stage can be found in their [official docs online]: http://openocd.org/doc/html/Server-Configuration.html


## Cargo problems
>>>>>>> upstream/master:f3discovery/src/appendix/1-general-troubleshooting/README.md

### "can't find crate for `core`"

<!-- #### Symptoms -->

#### 症状

```
   Compiling volatile-register v0.1.2
   Compiling rlibc v1.0.0
   Compiling r0 v0.1.0
error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

Build failed, waiting for other jobs to finish...
Build failed, waiting for other jobs to finish...
error: Could not compile `r0`.

To learn more, run the command again with --verbose.
```

<!-- #### Cause -->

#### 原因

<!-- 
You are using a toolchain older than `nightly-2018-04-08` and forgot to call `rustup target add
thumbv7em-none-eabihf`.
 -->

`nightly-2018-04-08`より古いツールチェインを使っており、`rustup target add thumbv7em-none-eabihf`の実行を忘れています。

<!-- #### Fix -->

#### 解決策

<!-- Update your nightly and install the `thumbv7em-none-eabihf` target. -->

nightlyを更新し、`thumbv7em-none-eabihf`ターゲットをインストールします。

``` console
$ rustup update nightly

$ rustup target add thumbv7em-none-eabihf
```
