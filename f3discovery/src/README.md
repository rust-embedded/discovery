<<<<<<< HEAD
# Discovery

<!-- > Discover the world of microcontrollers through [Rust]! -->

> [Rust]でマイクロコントローラの世界を楽しもう！

[Rust]: https://www.rust-lang.org/en-US/

<!-- 
This book is an introductory course on microcontroller-based embedded systems that uses Rust as the
teaching language rather than the usual C/C++.
 -->

この本は、よくあるC/C++ではなく、Rustを使ったマイクロコントローラの組込みシステム入門コースです。

<!-- ## Scope -->

## スコープ

<!-- The following topics will be covered (eventually, I hope): -->

以下のトピックを取り上げます（ゆくゆくは、そうしたいです）

<!-- - How to write, build, flash and debug an "embedded" (Rust) program. -->

- 「組込み」（Rust）プログラムの書き方、ビルド方法、フラッシュへの書き込み方法、デバッグ方法。

<!-- 
- Functionality ("peripherals") commonly found in microcontrollers: Digital input and output, Pulse
  Width Modulation (PWM), Analog to Digital Converters (ADC), common communication protocols like
  Serial, I2C and SPI, etc.
 -->

- マイクロコントローラで一般的な機能（「ペリフェラル」）。デジタル入出力、パルス幅変調（PWM）、アナログデジタル変換（ADC）、
  シリアル、I2C、SPIのような一般的な通信プロトコル、など。

<!-- - Multitasking concepts: cooperative vs preemptive multitasking, interrupts, schedulers, etc. -->

- マルチタスク。協調的マルチタスク vs プリエンプティブマルチタスク、割り込み、スケジューラなど。

<!-- 
- Control systems concepts: sensors, calibration, digital filters, actuators, open loop control,
  closed loop control, etc.
 -->

- 制御システム。センサ、キャリブレーション、デジタルフィルタ、アクチュエータ、開ループ制御、閉ループ制御、など。

<!-- ## Approach -->

## 進め方

<!-- - Beginner friendly. No previous experience with microcontrollers or embedded systems is required. -->

- 初心者に優しく。マイクロコントローラや組込みシステムの開発経験は必要ありません。

<!-- 
- Hands on. Plenty of exercises to put the theory into practice. *You* will be doing most of the
  work here.
 -->

- ハンズオン形式で。理論を実践するためにたくさんの演習をします。*あなた*はほとんどの作業をここで行います。

<!-- 
- Tool centered. We'll make plenty use of tooling to ease development. "Real" debugging, with GDB,
  and logging will be introduced early on. Using LEDs as a debugging mechanism has no place here.
 -->

- ツール中心に。開発を容易にするツールをたくさん使用します。GDBを使った「実際の」デバッグとログ出力を早い段階で導入します。
  デバッグ機能としてLEDを使用するようなことは、ここではやりません。

<!-- ## Non-goals -->

## 目標としないこと

<!-- What's out of scope for this book: -->

この本でスコープ外のことは、以下の通りです。

<!-- 
- Teaching Rust. There's plenty of material on that topic already. We'll focus on microcontrollers
  and embedded systems.
 -->

- Rustを教えること。このトピックについては、既に多くの教材があります。マイクロコントローラと組込みシステムに集中します。

<!-- 
- Being a comprehensive text about electric circuit theory or electronics. We'll just cover the
  minimum required to understand how some devices work.
 -->

- 電気回路または電子機器の理論についての包括的なテキストであること。
  いくつかのデバイスがどのように動くか、を理解するための最低限の情報を提供します。

<!-- 
- Covering Rustic, low level details. We won't be talking about linker scripts, the boot process or
  how to glue those two into a minimally working Rust program.
 -->

- Rustの低レベルな詳細を説明すること。リンカスクリプトやブートプロセス、
  また、最小限のRustプログラムにこれらの2つの要素を結合する方法については、説明しません。

<!-- 
Also I don't intend to port this material to other development boards; this book will make exclusive
use of the STM32F3DISCOVERY development board.
 -->

また、この教材を他の開発ボードに移植するつもりもありません。この本は、STM32F3DISCOVERY開発ボード専用のものです。

<!-- ## Reporting problems -->

## 問題の報告

<!-- 
The source of this book is in [this repository]. If you encounter any typo or problem with the code
report it on the [issue tracker].
 -->

この本のソースは[このレポジトリ]にあります。誤植やコードに問題を発見した場合は、[issueトラッカー]に報告して下さい。

<!-- 
[this repository]: https://github.com/rust-embedded/discovery
[issue tracker]: https://github.com/rust-embedded/discovery/issues
 -->

[このレポジトリ]: https://github.com/rust-embedded/discovery
[issueトラッカー]: https://github.com/rust-embedded/discovery/issues

> 訳注：和訳への問題報告は、下記にお願いいたします。

和訳のソースは[和訳レポジトリ]にあります。問題を発見した場合は、[和訳issue]に報告して下さい。

[和訳レポジトリ]: https://github.com/tomoyuki-nakabayashi/discovery
[和訳issue]: https://github.com/tomoyuki-nakabayashi/discovery/issues

<!-- ## Other embedded Rust resources -->

## 他の組込みRustの資料

<!-- 
This Discovery book is just one of several embedded Rust resources provided by the
[Embedded Working Group]. The full selection can be found at [The Embedded Rust Bookshelf]. This
includes the list of [Frequently Asked Questions].
 -->

このDiscovery本は、[組込みワーキンググループ]が提供する組込みRust資料の1つに過ぎません。
[組込みRustの本棚]に、数多くの資料があります。そこには、[よくある質問と回答]のリストも有ります。

<!-- 
[Embedded Working Group]: https://github.com/rust-embedded/wg
[The Embedded Rust Bookshelf]: https://docs.rust-embedded.org
[Frequently Asked Questions]: https://docs.rust-embedded.org/faq.html
 -->

[組込みワーキンググループ]: https://github.com/rust-embedded/wg
[組込みRustの本棚]: https://docs.rust-embedded.org
[よくある質問と回答]: https://docs.rust-embedded.org/faq.html

<!-- スポンサー紹介のため、あえて和訳していません。 -->

## Sponsored by

<p align="center">
<a href="http://integer32.com/">
<img style="width: 50%" title="integer 32" src="assets/integer32.svg">
</a>
</p>

Many thanks to [integer 32](http://integer32.com/) for sponsoring me to work on this book! Please
give them lots of work (they do Rust consulting!) so they'll have no choice but to hire more
Rustaceans <3.
=======
# `Discovery`

Discover the world of microcontrollers through [Rust](https://www.rust-lang.org/)!

There are currently two versions of this book. The first is older and uses an
F3 Discovery circuit board to introduce you to microcontrollers and Rust, while
the second is newer and uses a micro:bit circuit board instead.

- [Read the newer book, using a micro:bit](https://docs.rust-embedded.org/discovery/microbit)
- [Read the older book, using an F3 discovery board](https://docs.rust-embedded.org/discovery/f3discovery)
- Start working on the examples from this repository
- You've got questions?
    - Have a look at our [discussions section on
      GitHub](https://github.com/rust-embedded/discovery/discussions)
    - Maybe it has already been answered
    - If not, start a new discussion
- You've found an issue?
    - Have a look at our [issues on
      GitHub](https://github.com/rust-embedded/discovery/issues)
    - Maybe there is already a workaround
    - If not, please open a new one - or even better - a [pull
      request](https://github.com/rust-embedded/discovery/pulls) for solving
      it
- Have fun and enjoy!
>>>>>>> upstream/master
