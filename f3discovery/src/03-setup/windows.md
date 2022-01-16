# Windows

## `arm-none-eabi-gdb`

<!-- 
ARM provides `.exe` installers for Windows. Grab one from [here][gcc], and follow the instructions.
Just before the installation process finishes tick/select the "Add path to environment variable"
option. Then verify that the tools are in your `%PATH%`:
 -->

ARMはWindows向けに`.exe`インストーラを提供しています。[ここ][gcc]から1つを入手して、手順に従って下さい。
インストールプロセスが終了する直前に"環境変数にパスを追加"オプションを選択します。
その後、ツールが`%PATH%`にあることを確認します。

Verify gcc is installed:
``` console
arm-none-eabi-gcc -v
```
The results should be something like:
```
(..)
$ arm-none-eabi-gcc -v
gcc version 5.4.1 20160919 (release) (..)
```

[gcc]: https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads

## OpenOCD

<!-- 
There's no official binary release of OpenOCD for Windows but there are unofficial releases
available [here][openocd]. Grab the 0.10.x zipfile and extract it somewhere in your drive (I
recommend `C:\OpenOCD` but with the drive letter that makes sense to you) then update your `%PATH%`
environment variable to include the following path: `C:\OpenOCD\bin` (or the path that you used
before).
 -->

Windows用のOpenOCDの公式バイナリはありませんが、[こちら][openocd]の非公式なリリースが利用可能です。
0.10.xのzipファイルを入手し、ハードディスクのどこかに展開して下さい(`C:\OpenOCD`をお勧めしますが、あなたが分かるドライブ名を使用してください)。
その後、`%PATH%`環境変数が`C:\OpenOCD\bin`(もしくはあなたがOpenOCDのzipファイルを展開したパス)を含むように更新します。

[openocd]: https://github.com/gnu-mcu-eclipse/openocd/releases
<!-- 
Verify that OpenOCD is in yout `%PATH%` with: -->

OpenOCDが`%PATH%`にあることを確認します。

Verify OpenOCD is installed and in your `%PATH%` with:
``` console
openocd -v
```
The results should be something like:
``` console
$ openocd -v
Open On-Chip Debugger 0.10.0
(..)
```

## PuTTY

<!-- Download the latest `putty.exe` from [this site] and place it somewhere in your `%PATH%`. -->

[このサイト]から最新の`putty.exe`をダウンロードし、`%PATH%`が通っているどこかに置いて下さい。

<!-- [this site]: http://www.chiark.greenend.org.uk/~sgtatham/putty/download.html -->

[このサイト]: http://www.chiark.greenend.org.uk/~sgtatham/putty/download.html

## ST-LINK USB driver

<!-- 
You'll also need to install [this USB driver] or OpenOCD won't work. Follow the installer
instructions and make sure you install the right (32-bit or 64-bit) version of the driver.
 -->

[このUSBドライバ]もインストールする必要があります。そうでなければOpenOCDは動きません。インストーラの手順に従って下さい。
そして、正しいドライバのバージョン(32ビットか64ビット)をインストールすることを確認して下さい。

<!-- [this USB driver]: http://www.st.com/en/embedded-software/stsw-link009.html -->

[このUSBドライバ]: http://www.st.com/en/embedded-software/stsw-link009.html

<!-- That's all! Go to the [next section]. -->

以上です！[次のセクション]に進んで下さい。

<!-- [next section]: verify.md -->

[次のセクション]: verify.md
