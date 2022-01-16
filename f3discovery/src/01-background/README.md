<!-- # Background -->

# 背景

<!-- ## What's a microcontroller? -->

## マイクロコントローラとは何でしょうか？

<!-- 
A microcontroller is a *system* on a chip. Whereas your laptop is made up of several discrete
components: a processor, RAM sticks, a hard drive, an ethernet port, etc.; a microcontroller has all
those components built into a single "chip" or package. This makes it possible to build systems with
minimal part count.
 -->

マイクロコントローラは、1チップ上の*システム*です。
一方、あなたのノートPCは、プロセッサ、RAMスティック、ハードディスクドライブ、イーサーネットポートなど、いくつかの個別の部品で構成されています。
マイクロコントローラは、それらの構成部品を1つの「チップ」またはパッケージ内に組み込みます。
このことにより、最小限の部品数からなるシステムを構築することが可能になります。

<!-- ## What can you do with a microcontroller? -->

## マイクロコントローラで何ができるのでしょうか？

<!-- 
Lots of things! Microcontrollers are the central part of systems known as *embedded* systems. These
systems are everywhere but you don't usually notice them. These systems control the brakes of your
car, wash your clothes, print your documents, keep you warm, keep you cool, optimize the fuel
consumption of your car, etc.
 -->

たくさんのことができます！マイクロコントローラは*組込み*システムとして知られるシステムの中心を担う部品です。
組込みシステムはどこにでもありますが、通常それらを意識することはありません。組込みシステムは車のブレーキを制御したり、
衣服を洗濯したり、ドキュメントを印刷したり、冷暖房を制御したり、車の燃料消費を最適化したりします。

<!-- 
The main trait of these systems is that they operate without user intervention even if they expose a
user interface like a washing machine does; most of their operation is done on their own.
 -->

組込みシステムの主な特徴は、洗濯機のようにユーザーインタフェースがある場合でさえ、ユーザーの介入なしに動作することです。
ほとんどの動作は、組込みシステム自身で完結します。

<!-- 
The other common trait of these systems is that they *control* a process. And for that these systems
usually have one or more sensors and one or more actuators. For example, an HVAC system has several
sensors, thermometers and humidity sensors spread across some area, and several actuators as well,
heating elements and fans connected to ducts.
 -->

組込みシステムの他の特徴としては、プロセスを*制御*することです。このために組込みシステムは、通常、1つ以上のセンサとアクチュエータを持ちます。
例えば、空調システムは、点在する温度計や湿度計といったセンサと、発熱体やダクトに接続されたファンのようなアクチュエータを持ちます。

<!-- ## When should I use a microcontroller? -->

## マイクロコントローラを使うべきなのはいつでしょうか？

<!-- 
All these application I've mentioned, you can probably implement with a Raspberry Pi, a computer
that runs Linux. Why should I bother with a microcontroller that operates without an OS? Sounds like
it would be harder to develop a program.
 -->

上述した全てのアプリケーションを、Linuxが動作するRaspberry Piで実装することは可能でしょう。
わざわざOSなしで動作するマイクロコントローラを使うのはなぜでしょうか？
プログラムを開発するのは、容易ではなさそうです。

<!-- 
The main reason is cost. A microcontroller is much cheaper than a general purpose computer. Not only
the microcontroller is cheaper; it also requires many fewer external electrical components to
operate. This makes Printed Circuit Boards (PCB) smaller and cheaper to design and manufacture.
 -->

主な理由はコストです。マイクロコントローラは、汎用コンピュータより非常に安価です。マイクロコントローラ自体が安いだけではないです。
マイクロコントローラは、動作のための外部電気部品を少ししか必要としません。そのため、プリント基板（PCB）を小さく、安価に設計、製造できます。

<!-- 
The other big reason is power consumption. A microcontroller consumes orders of magnitude less power
than a full blown processor. If your application will run on batteries that makes a huge difference.
 -->

他の大きな理由は、消費電力です。マイクロコントローラは、本格的なプロセッサよりも数桁少ない電力しか消費しません。
アプリケーションがバッテリで動作するとき、この違いは大きいです。

<!-- 
And last but not least: (hard) *real time* constraints. Some processes require their controllers to
respond to some events within some time interval (e.g. a quadcopter/drone hit by a wind gust). If
this *deadline* is not met, the process could end in catastrophic failure (e.g. the drone crashes to
the ground). A general purpose computer  running a general purpose OS has many services running in
the background. This makes it hard to guarantee execution of a program within tight time constraints.
 -->

最後になりましたが、（ハード）*リアルタイム*制約があるためです。
プロセスによっては、ある時間間隔以内にイベントに応答する必要があります（例えば、クアッドコプター/ドローンが突風に襲われた場合）。
もし、*デッドライン*を満たさない場合、そのプロセスは悲惨な結末を迎えるでしょう（例えば、ドローンは地面に墜落します）。
汎用OSを実行している汎用コンピュータは、背後で多くのサービスが動作しています。
このことは、厳密な時間制約内でのプログラム実行の保証を難しくします。

<!-- ## When should I *not* use a microcontroller? -->

## マイクロコントローラを使うべきで*ない*時はいつでしょうか？

<!-- 
Where heavy computations are involved. To keep their power consumption low, microcontrollers have
very limited computational resources available to them. For example, some microcontrollers don't
even have hardware support for floating point operations. On those devices, performing a simple
addition of single precision numbers can take hundreds of CPU cycles.
 -->

計算量が膨大な場合です。消費電力を低くするため、マイクロコントローラは非常に限られた計算資源しか持っていません。
例えば、マイクロコントローラによっては、浮動小数点演算のハードウェアすら搭載していません。
そのようなデバイスでは、単精度の単純な加算でさえも、実行に数百CPUサイクルかかるでしょう。

<!-- ## Why use Rust and not C? -->

## CではなくRustを使う理由はなんでしょうか？

<!-- 
Hopefully, I don't need to convince you here as you are probably familiar with the language
differences between Rust and C. One point I do want to bring up is package management. C lacks an
official, widely accepted package management solution whereas Rust has Cargo. This makes development
*much* easier. And, IMO, easy package management encourages code reuse because libraries can be
easily integrated into an application which is also a good thing as libraries get more "battle
testing".
 -->

あなたはRustとCとの違いを知っており、ここで説得する必要がないことを願っています。
あえて、1つ強調すると、それはパッケージ管理です。RustにはCargoがある一方、Cは公式の広く普及しているパッケージ管理システムがありません。
Cargoは開発を*非常に*容易にします。私の意見としては、パッケージ管理が簡単であることは、コードの再利用を促進します。
なぜなら、ライブラリがアプリケーションに容易に結合できるからです。このことは、ライブラリがより「実戦で使われる」ことにも良い影響があります。

<!-- ## Why should I not use Rust? -->

## Rustを使うべきでない理由は何でしょうか？

<!-- Or why should I prefer C over Rust? -->

もしくは、RustよりCを選ぶ理由はなんでしょうか？

<!-- 
The C ecosystem is way more mature. Off the shelf solution for several problems already exist. If
you need to control a time sensitive process, you can grab one of the existing commercial Real Time
Operating Systems (RTOS) out there and solve your problem. There are no commercial, production-grade
RTOSes in Rust yet so you would have to either create one yourself or try one of the ones that are
in development.
 -->

Cのエコシステムはより成熟しています。いくつもの問題に対する解決策が既に存在しています。
時間制約のあるプロセスを制御する必要がある場合、既存の商用リアルタイムOS（RTOS）を選び、問題を解決することができます。
Rustにはまだ、商用で製品レベルのRTOSがないため、自分自身で作るか、開発中のものを試す必要があります。
