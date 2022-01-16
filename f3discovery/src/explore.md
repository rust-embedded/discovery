<!-- # What's left for you to explore -->

# もっと楽しむために

<!-- We have barely scratched the surface! There's lots of stuff left for you to explore: -->

まだ上っ面をなでただけです！まだまだたくさん楽しめるものがあります。

<!-- ## Multitasking -->

## マルチタスク

<!-- 
All our programs executed a single task. How could we achieve multitasking in a system with no OS,
and thus no threads. There are two main approaches to multitasking: preemptive multitasking and
cooperative multitasking.
 -->

作ったプログラムは全てシングルタスクで実行しました。OSがなく、スレッドがないシステムでマルチタスクを実現するにはどうするのでしょうか。
2つの主なアプローチがあります。プリエンプティブマルチタスクと協調的マルチタスクです。

<!-- 
In preemptive multitasking a task that's currently being executed can, at any point in time, be
*preempted* (interrupted) by another task. On preemption, the first task will be suspended and the
processor will instead execute the second task. At some point the first task will be resumed.
Microcontrollers provide hardware support for preemption in the form of *interrupts*.
 -->

プリエンプティブマルチタスクでは、実行されているタスクは、いつでも、他のタスクによって*プリエンプション*（割込み）されます。
プリエンプションでは、最初のタスクは一時停止され、プロセッサは2つ目のタスクを代わりに実行します。
どこかの時点で最初のタスクが再開されます。
マイクロコントローラは、プリエンプションを*割込み*という形でハードウェアサポートしています。

<!-- 
In cooperative multitasking a task that's being executed will run until it reaches a *suspension
point*. When the processor reaches that suspension point it will stop executing the current task and
instead go and execute a different task. At some point the first task will be resumed. The main
difference between these two approaches to multitasking is that in cooperative multitasking *yields*
execution control at *known* suspension points instead of being forcefully preempted at any point of
its execution.
 -->

協調的マルチタスクでは、実行されているタスクは*中断点*に到達するまで実行します。
プロセッサが中断点に到達すると、現在のタスクの実行を停止し、代わりに別のタスクを実行します。どこかの時点で最初のタスクが再開されます。
マルチタスクのための2つのアプローチの主な違いは、いかなるタイミングでも強制的にその実行がプリエンプションされる代わりに、
協調的マルチタスクでは*既知の*中断点で実行制御を*譲る*ことです。

## Direct Memory Access (DMA).

<!-- 
This peripheral is a kind of *asynchronous* `memcpy`. So far our programs have
been pumping data, byte by byte, into peripherals like UART and I2C. This DMA
peripheral can be used to perform bulk transfers of data. Either from RAM to
RAM, from a peripheral, like a UART, to RAM or from RAM to a peripheral. You can
schedule a DMA transfer, like read 256 bytes from USART1 into this buffer, leave
it running in the background and then poll some register to see if it has
completed so you can do other stuff while the transfer is ongoing.
 -->

このペリフェラルは、*非同期*の`memcpy`です。これまでのところ、プログラムは、UARTやI2Cのようなペリフェラルに、バイトごとにデータを送信してきました。
DMAペリフェラルは、データの一括転送を行うために使用できます。RAMからRAM、UARTのようなペリフェラルからRAM、RAMからペリフェラルへ転送できます。
DMA転送をスケジュールすることもできます。
例えば256バイトをUSART1からバッファへ読み込む、という処理をバックグランドで実行しながら、転送が完了したかをレジスタをポーリングすることで、
転送を行っている間、他のことができます。

<!-- ## Sleeping -->

## スリープ

<!-- 
All our programs have been continuously polling peripherals to see if there's
anything that needs to be done. However, some times there's nothing to be done!
At those times, the microcontroller should "sleep".
 -->

作った全てのプログラムは、必要な処理が終わったかどうかを見るために、ペリフェラルを継続的にポーリングしています。
しかし、なにもすることがないことがあります。
そのような場合、マイクロコントローラは「スリープ」しなければなりません。

<!-- 
When the processor sleeps, it stops executing instructions and this saves power.
It's almost always a good idea to save power so your microcontroller should be
sleeping as much as possible. But, how does it know when it has to wake up to
perform some action? "Interrupts" are one of the events that wake up the
microcontroller but there are others and the `wfi` and `wfe` are the
instructions that make the processor "sleep".
 -->

プロセッサがスリープしている時、命令の実行を停止し、電力を節約します。
これは、電力節約のために、多くの場合、良いことです。マイクロコントローラは可能な限りスリープしなければなりません。
しかし、何か処理を行うためにいつ目覚めるか、をどうやって知ればよいでしょうか？
「割込み」はマイクロコントローラを目覚めさせるイベントの1つですが、他の方法もあります。
`wfi`と`wfe`はプロセッサを「スリープ」させる命令です。

<!-- ## Pulse Width Modulation (PWM) -->

## パルス幅変調（PWM）

<!-- 
In a nutshell, PWM is turning on something and then turning it off periodically
while keeping some proportion ("duty cycle") between the "on time" and the "off
time". When used on a LED with a sufficiently high frequency, this can be used
to dim the LED. A low duty cycle, say 10% on time and 90% off time, will make
the LED very dim wheres a high duty cycle, say 90% on time and 10% off time,
will make the LED much brighter (almost as if it were fully powered).
 -->

手短に言うと、PWMは、周期的に何かをオンにして、その後オフにすることです。
ここで、「オンの時間」と「オフの時間」の間は、ある割合（「デューティ比」）を保ち続けます。
十分周波数の高いLEDを使用すると、PWMは、LEDを薄暗くするために使用できます。
ディーティ比が低いと、つまり10%オンで90%オフだと、LEDは非常に薄暗くなります。反対に、デューティ比が高いと、
つまり90%オンで10%オフだと、LEDはかなり明るくなります（ほとんど完全に電力を供給されたように）。

<!-- 
In general, PWM can be used to control how much *power* is given to some
electric device. With proper (power) electronics between a microcontroller and
an electrical motor, PWM can be used to control how much power is given to the
motor thus it can be used to control its torque and speed. Then you can add an
angular position sensor and you got yourself a closed loop controller that can
control the position of the motor at different loads.
 -->

通常、PWMは電気機器にどの程度*電力*を供給するか、制御するために使われます。
マイクロコントローラと電気モータとの間に、適切な（電力）電子機器を用いて、PWMでモータにどれだけの電力供給するか制御できます。
そのため、PWMを使用してモータのトルクと速度を制御できます。
それから、角位置センサを追加し、閉ループコントローラを入手できます。
このコントローラは、様々な負荷でモータの位置を制御することができます。

<!-- ## Digital input -->

## デジタル入力

<!-- 
We have used the microcontroller pins as digital outputs, to drive LEDs. But
these pins can also be configured as digital inputs. As digital inputs, these
pins can read the binary state of switches (on/off) or buttons (pressed/not
pressed).
 -->

LEDを駆動するために、マイクロコントローラのピンをデジタル出力として使用してきました。
しかし、これらのピンはデジタル入力として設定することもできます。
デジタル入力として、これらのピンは、スイッチ（オン/オフ）やボタン（押された/押されていない）の二値状態を読むことができます。

<!-- 
(*spoilers* reading the binary state of switches / buttons is not as
straightforward as it sounds ;-)
 -->

（*ネタバレ* スイッチ/ボタンの二値状態を読むことは、思ったほど簡単ではありません ;-)）

<!-- ## Sensor fusion -->

## センサフュージョン

<!-- 
The STM32F3DISCOVERY contains three motion sensors: an accelerometer, a
gyroscope and a magnetometer. On their own these measure: (proper) acceleration,
angular speed and (the Earth's) magnetic field. But these magnitudes can be
"fused" into something more useful: a "robust" measurement of the orientation of
the board. Where robust means with less measurement error than a single sensor
would be capable of.
 -->

STM32F3DISCOVERYは3つのモーションセンサを搭載しています。加速度計、ジャイロスコープ、磁力計です。
彼らは(proper)加速度、角速度、（地球）磁場を計測できます。
しかし、これらの測定値は、「ロバストな」ボード向きの測定のように、もっと有用なものに「融合」することができます。
ここでロバストが意味するところは、単一のセンサで計測できるものより、計測誤差が少ないことです。

<!-- 
This idea of deriving more reliable data from different sources is known as
sensor fusion.
 -->

より信頼性の高いデータを、異なるソースから生み出すためのこのアイデアは、センサフュージョンと呼ばれています。

<!-- ## Analog-to-Digital Converters (ADC) -->

## アナログデジタルコンバータ（ADC）

<!-- 
There are a lots of digital sensors out there. You can use a protocol like I2C
and SPI to read them. But analog sensors also exist! These sensors just output a
voltage level that's proportional to the magnitude they are sensing.
 -->

たくさんのデジタルセンサがあります。I2CやSPIのようなプロトコルを使って、センサからデータを読み出せます。
しかし、アナログセンサも存在しています！これらのセンサは、検知している大きさに比例した電圧レベルを出力するだけです。

<!-- 
The ADC peripheral can be use to convert that "analog" voltage level, say `1.25`
Volts,into a "digital" number, say in the `[0, 65535]` range, that the processor
can use in its calculations.
 -->

ADCペリフェラルは、「アナログ」電圧を変換するために使用します。
つまり`1.25`ボルトを、プロセッサが計算で使える「デジタル」な数値である`[0, 65535]`の範囲に変換します。

<!-- ## Digital-to-Analog Converters (DAC) -->

## デジタルアナログコンバータ（DAC）

<!-- 
As you might expect a DAC is exactly the opposite of ADC. You can write some
digital value into a register to produce a voltage in the `[0, 3.3V]` range
(assuming a `3.3V` power supply) on some "analog" pin. When this analog pin is
connected to some appropriate electronics and the register is written to at some
constant, fast rate (frequency) with the right values you can produce sounds or
even music!
 -->

予想の通り、DACはADCの反対のことを行います。
何らかのデジタル値をレジスタに書き込むと、`[0, 3.3V]`の範囲の電圧を、「アナログ」ピンに出力します（電源が`3.3V`と仮定しています）。
このアナログピンが、何らかの適切な電子機器に接続されており、何らかの定数がレジスタに書き込まれると、
正しい値の速いレート（周波数）は、サウンドや音楽さえも生み出すことができます。

<!-- ## Real Time Clock (RTC) -->

## リアルタイムクロック（RTC）

<!-- 
This peripheral can be used to track time in "human format". Seconds, minutes,
hours, days, months and years. This peripheral handles the translation from
"ticks" to these human friendly units of time. It even handles leap years and
Daylight Save Time for you!
 -->

このペリフェラルは、「人間の形式」で時間を追跡するために使用できます。
秒、分、時間、日、月、年の形式です。このペリフェラルは、「ティック」から人間が読みやすい時間の単位への変換を取り扱います。
RTCは、うるう年や夏時間でさえも取り扱います！

<!-- ## Other communication protocols -->

## 他の通信プロトコル

<!-- SPI, I2S, SMBUS, CAN, IrDA, Ethernet, USB, Bluetooth, etc. -->

SPI, I2S, SMBUS, CAN, IrDA, Ethernet, USB, Bluetooth,など。

<!-- 
Different applications use different communication protocols. User facing
applications usually have an USB connector because USB is an ubiquitous
protocol in PCs and smartphones. Whereas inside cars you'll find plenty of CAN
"buses". Some digital sensors use SPI, others use I2C and others, SMBUS.
 -->

アプリケーションごとに違う通信プロトコルを使います。ユーザーが触れるアプリケーションは、通常USBコネクターを持ちます。
なぜなら、USBは、PCやスマートフォンの至るところで使われているプロトコルだからです。
一方、車の内部では、多くのCAN「バス」を見つけられます。デジタルセンサにはSPIを使うものがあり、他のものはI2CやSMBUSを使います。

## General Embedded-Relevant Topics

These topics cover items that are not specific to our device, or the hardware on
it. Instead, they discuss useful techniques that could be used on embedded
systems.

### Gyroscopes

As part of our Punch-o-meter exercise, we used the Accelerometer to measure
changes in acceleration in three dimensions. Our board also features a sensor
called a Gyroscope, which allows us to measure changes in "spin" in three
dimensions.

This can be very useful when trying to build certain systems, such as a robot
that wants to avoid tipping over. Additionally, the data from a sensor like a
gyroscope can also be combined with data from accelerometer using a technique
called Sensor Fusion (see below for more information).

### Servo and Stepper Motors

While some motors are used primarily just to spin in one direction or the other,
for example driving a remote control car forwards or backwards, it is sometimes
useful to measure more precisely how a motor rotates.

Our microcontroller can be used to drive Servo or Stepper motors, which allow
for more precise control of how many turns are being made by the motor, or
can even position the motor in one specific place, for example if we wanted to
move the arms of a clock to a particular direction.

### Sensor fusion

The STM32F3DISCOVERY contains three motion sensors: an accelerometer, a
gyroscope and a magnetometer. On their own these measure: (proper) acceleration,
angular speed and (the Earth's) magnetic field. But these magnitudes can be
"fused" into something more useful: a "robust" measurement of the orientation of
the board. Where robust means with less measurement error than a single sensor
would be capable of.

This idea of deriving more reliable data from different sources is known as
sensor fusion.

---

<!-- So where to next? There are several options: -->

さて、次はどこでしょう？いくつかの選択肢があります。

<!-- 
- You could check out the examples in the [`f3`] board support crate. All those examples work for
  the STM32F3DISCOVERY board you have.
 -->

- [`f3`]ボードサポートクレートにある例をチェックできます。全ての例は、STM32F3DISCOVERYボードで動きます。

[`f3`]: https://docs.rs/f3

<!-- 
- You could try out [this motion sensors demo][madgwick]. Details about the implementation and
  source code are available in [this blog post][wd-1-2].
 -->

- [このモーションセンサデモ][madgwick]を試すことができます。実装とソースコードについての詳細は、[このブログ][wd-1-2]にあります。

[madgwick]: https://mobile.twitter.com/japaricious/status/962770003325005824
[wd-1-2]: http://blog.japaric.io/wd-1-2-l3gd20-lsm303dlhc-madgwick/

<!-- 
- You could check out [Real Time for The Masses]. A very efficient preemptive multitasking framework
  that supports task prioritization and dead lock free execution.
 -->

- [Real Time for The Masses]をチェックできます。非常に効率的なプリエンプティブマルチタスクフレームワークです。
  これは、タスクの優先度と、デッドロックしない実行を提供します。

[Real Time for The Masses]: https://docs.rs/cortex-m-rtfm

<!-- 
- You could try running Rust on a different development board. The easiest way to get started is to
  use the [`cortex-m-quickstart`] Cargo project template.
 -->

- Rustを別の開発ボードで動かしてみることができます。最も簡単な方法は、[`cortex-m-quickstart`]のCargoプロジェクトテンプレートを使うことです。

[`cortex-m-quickstart`]: https://docs.rs/cortex-m-quickstart/0.2.4/cortex_m_quickstart

<!-- 
- You could check out [this blog post][brave-new-io] which describes how Rust type system can
  prevent bugs in I/O configuration.
 -->

- [このブログ][brave-new-io]をチェックできます。このブログは、Rustの型システムがI/O設定のバグを防ぐ方法を説明しています。

[brave-new-io]: http://blog.japaric.io/brave-new-io/

<!-- - You could check out my [blog] for miscellaneous topics about embedded development with Rust. -->

- 私の[ブログ][blog]をチェックできます。Rustを使った組込み開発について、様々なトピックを書いています。

[blog]: http://blog.japaric.io

<!-- 
- You could check out the [`embedded-hal`] project which aims to build abstractions (traits) for all
  the embedded I/O functionality commonly found on microcontrollers.
 -->

- [`embedded-hal`]プロジェクトをチェックできます。このプロジェクトは、
  マイクロコントローラで一般的に用いられる組込みI/O機能に対して、抽象化（トレイト）を構築することを目指しています。

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

<!-- 
- You could join the [Weekly driver initiative] and help us write generic drivers on top of the
  `embedded-hal` traits and that work for all sorts of platforms (ARM Cortex-M, AVR, MSP430, RISCV,
  etc.)
 -->

- [Weekly driver initiative]に参加して、`embedded-hal`トレイト上で汎用ドライバを書く手伝いができます。
  この汎用ドライバは、あらゆる種類のプラットフォーム（ARM Cortex-M、AVR、MSP430、RISCVなど）で動作します。

[Weekly driver initiative]: https://github.com/rust-lang-nursery/embedded-wg/issues/39
