<!-- # Verify the installation -->

# インストールの確認

<!-- Let's verify that all the tools were installed correctly. -->

全てのツールが正しくインストールされているか、確認しましょう。

<!-- ## Linux only -->

## Linuxのみ実行

<!-- ### Verify permissions -->

### パーミッションの確認

<!-- 
Connect the F3 to your laptop using an USB cable. Be sure to connect the cable to the "USB ST-LINK"
port, the USB port in the center of the edge of the board.
 -->

USBケーブルを使って、ノートPCをF3に接続して下さい。
ボード端の中央にある「USB ST-LINK」とラベルが付いたものにケーブルを接続して下さい。

<!-- 
The F3 should now appear as a USB device (file) in `/dev/bus/usb`. Let's find out how it got
enumerated:
 -->

これで、F3が`/dev/bus/usb`にUSBデバイス（ファイル）として現れるはずです。
どのように番号付けされるか調べてみましょう。

``` console
$ lsusb | grep -i stm
Bus 003 Device 004: ID 0483:374b STMicroelectronics ST-LINK/V2.1
$ # ^^^        ^^^
```

<!-- 
In my case, the F3 got connected to the bus #3 and got enumerated as the device #4. This means the
file `/dev/bus/usb/003/004` *is* the F3. Let's check its permissions:
 -->

私の場合、F3は、3番目のバスに接続されて、4番目のデバイスとして番号付けされています。
これは、`/dev/bus/usb/003/004`*が*F3であることを意味します。パーミッションを確認しましょう。

``` console
$ ls -l /dev/bus/usb/003/004
crw-rw-rw- 1 root root 189, 20 Sep 13 00:00 /dev/bus/usb/003/004
```

<!-- 
The permissions should be `crw-rw-rw-`. If it's not ... then check your [udev
rules] and try re-loading them with:
 -->

パーミッションは、`crw-rw-rw-`でなければなりません。もし違う場合、[udevルール]を確認し、次のコマンドでリロードしてみて下さい。

<!-- [udev rules]: linux.md#udev-rules -->

[udevルール]: linux.md#udev-rules

``` console
$ sudo udevadm control --reload-rules
```

<!-- Now let's repeat the procedure for the Serial module. -->

上記の手順をシリアルモジュールについても繰り返します。

<!-- Unplug the F3 and plug the Serial module. Now, figure out what's its associated file: -->

F3を抜いて、シリアルモジュールを接続します。すると、関連するファイルが見つかります。

``` console
$ lsusb | grep -i ft232
Bus 003 Device 005: ID 0403:6001 Future Technology Devices International, Ltd FT232 Serial (UART) IC
```

<!-- In my case, it's the `/dev/bus/usb/003/005`. Now, check its permissions: -->

私の場合、`/dev/bus/usb/003/005`です。パーミッションを確認します。

``` console
$ ls -l /dev/bus/usb/003/005
crw-rw-r-- 1 root root 189, 21 Sep 13 00:00 /dev/bus/usb/003/005
```

<!-- As before, the permissions should be `crw-rw-r--`. -->

前回同様、パーミッションは、`crw-rw-r--`でなければなりません。

<!-- ## All -->

## 全てのOS

<!-- ### First OpenOCD connection -->

### 初めてのOpenOCD接続

<!-- 
First, connect the F3 to your laptop using an USB cable. Connect the cable to the USB port in the
center of edge of the board, the one that's labeled "USB ST-LINK".
 -->

USBケーブルを使って、ノートPCをF3に接続して下さい。
ボード端の中央にある「USB ST-LINK」とラベルが付いたものにケーブルを接続して下さい。

<!-- Two *red* LEDs should turn on right after connecting the USB cable to the board. -->

ボードにUSBケーブルを接続した直後、2つの*赤い*LEDが点灯するはずです。

<!-- Next, run this command: -->

次に、このコマンドを実行して下さい。

``` console
$ # *nix
$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg

$ # Windows
$ # 注記 cygwinユーザーは、-sフラグで問題があったと報告しています。もし、問題に遭遇した場合、
$ # `C:\OpenOCD\share\scripts`ディレクトリからopenocdを呼ぶことができます。
$ openocd -s C:\OpenOCD\share\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

<!-- > **NOTE** Windows users: `C:\OpenOCD` is the directory where you installed OpenOCD to. -->

> **注記** Windowsユーザーへ。`C:\OpenOCD`は、OpenOCDをインストールしたディレクトリです。

<!-- 
> **IMPORTANT** There is more than one hardware revision of the STM32F3DISCOVERY board. For older
> revisions, you'll need to change the "interface" argument to `-f interface/stlink-v2.cfg` (note:
> no `-1` at the end). Alternatively, older revisions can use `-f board/stm32f3discovery.cfg`
> instead of `-f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`.
 -->

> **重要** STM32F3DISCOVERYボードには、2つ以上のハードウェアリビジョンが存在します。古いリビジョンのハードウェアについては、
> 「interface」引数を`-f interface/stlink-v2.cfg`に変更する必要が有ります（`-1`が最後についていないことに留意して下さい）
> 代わりに、古いリビジョンでは、`-f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`の代わりに
> `-f board/stm32f3discovery.cfg`を使用できます。

<!-- You should see output like this: -->

次のような出力が確認できるはずです。

``` console
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
none separate
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v27 API v2 SWIM v15 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.915608
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

<!-- (If you don't ... then check the [general troubleshooting] instructions.) -->

（確認できない場合、[トラブルシューティング]の手順を確認して下さい。）

<!-- [general troubleshooting]: ../appendix/1-general-troubleshooting/index.html -->

[トラブルシューティング]: ../appendix/1-general-troubleshooting/index.html

<!-- `openocd` will block the terminal. That's fine. -->

`openocd`は端末をブロックします。それで問題ありません。

<!-- 
Also, one of the red LEDs, the one closest to the USB port, should start oscillating between red
light and green light.
 -->

また、赤いLEDの1つ（USBポートに一番近いもの）が、周期的に赤と緑に点灯し始めるはずです。

<!-- That's it! It works. You can now close/kill `openocd`. -->

これでおしまいです！ボードは動いています。それでは、`openocd`を終了できます。
