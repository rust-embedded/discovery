<!-- # Meet your hardware -->

# ハードウェアとの出会い

<!-- Let's get familiar with the hardware we'll be working with. -->

これから使用するハードウェアについて詳しくなりましょう。

<!-- ## STM32F3DISCOVERY (the "F3") -->

## STM32F3DISCOVERY（「F3」）

<p align="center">
<img title="F3" src="../assets/f3.jpg">
</p>

<!-- We'll refer to this board as "F3" throughout this book. -->

この本では、このボードを「F3」と呼びます。

<!-- What does this board contain? -->

このボードには何が搭載されているか見てみましょう。

<!--
- A STM32F303VCT6 microcontroller. This microcontroller has
  - A single core ARM Cortex-M4F processor with hardware support for single precision floating point
    operations and a maximum clock frequency of 72 MHz.
 -->
<!--   - 256 KiB of "Flash" memory. (1 KiB = 10**24** bytes) -->

<!--   - 48 KiB of RAM. -->

<!--   - many "peripherals": timers, GPIO, I2C, SPI, USART, etc. -->

<!--   - lots of "pins" that are exposed in the two lateral "headers". -->

<!--   - **IMPORTANT** This microcontroller operates at (around) 3.3V. -->

- STM32F303VCT6マイクロコントローラが1つ。このマイクロコントローラは次のものを搭載しています。
  - シングルコアのARM Cortex-M4Fプロセッサ。このプロセッサは単精度浮動小数点演算を行うハードウェアを搭載し、
    最大72MHzのクロック周波数で動作します。

  - 256 KiBの「Flash」メモリ（1 KiB = 10**24**バイト）

  - 48 KiBのRAM

  - 多くの「ペリフェラル」。タイマ、GPIO、I2C、SPI、USARTなど。

  - 2つの横方向の「ヘッダー」に露出しているたくさんの「ピン」

  - **重要** このマイクロコントローラは、（おおよそ）3.3Vで動作します。

<!-- - An [accelerometer] and a [magnetometer][] (in a single package). -->

- [加速度計]1つと[磁力計]1つ（1つのパッケージ内にまとめられています）。

<!-- 
[accelerometer]: https://en.wikipedia.org/wiki/Accelerometer
[magnetometer]: https://en.wikipedia.org/wiki/Magnetometer
 -->

[加速度計]: https://en.wikipedia.org/wiki/Accelerometer
[磁力計]: https://en.wikipedia.org/wiki/Magnetometer

<!-- - A [gyroscope]. -->

- [ジャイロスコープ]が1つ

<!-- [gyroscope]: https://en.wikipedia.org/wiki/Gyroscope -->

[ジャイロスコープ]: https://en.wikipedia.org/wiki/Gyroscope

<!-- - 8 user LEDs arranged in the shape of a compass -->

- 円形に配置された8個のユーザLED

<!-- 
- A second microcontroller: a STM32F103CBT. This microcontroller is actually part of an on-board
  programmer and debugger named ST-LINK and is connected to the USB port named "USB ST-LINK".
 -->

- 第2のマイクロコントローラ: STM32F103CBT。このマイクロコントローラは、実際には、ST-LINKというオンボードプログラマおよびデバッガの一部であり、
  「USB ST-LINK」という名前のUSBポートに接続されています。

<!-- 
- There's a second USB port, labeled "USB USER" that is connected to the main microcontroller, the
  STM32F303VCT6, and can be used in applications.
 -->

- 「USB USER」というラベルが付いている第2のUSBポート。
  このUSBポートは、メインマイクロコントローラ（STM32F303VCT6）に接続されており、アプリケーションで利用できます。

<!-- ## The Serial module -->

## シリアルモジュール

<p align="center">
<img title="Serial module" src="../assets/serial.jpg">
</p>

<!-- 
We'll use this module to exchange data between the microcontroller in the F3 and your laptop. This
module will be connected to your laptop using an USB cable. I won't say more at this point.
 -->

このモジュールは、F3のマイクロコントローラとノートPCとの間でデータをやり取りするために使います。
このモジュールは、USBケーブルを使ってノートPCに接続されます。ここでは、これ以上言及しません。

<!-- ## The Bluetooth module -->

## Bluetoothモジュール

<p align="center">
<img title="The HC-05 Bluetooth module" src="../assets/bluetooth.jpg">
</p>

<!-- 
This module has the exact same purpose as the serial module but it sends the data over Bluetooth
instead of over USB.
 -->

このモジュールは、シリアルモジュールと全く同じ目的で使いますが、データはUSBの代わりにBluetooth経由で送信します。
