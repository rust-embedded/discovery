<!-- # LED roulette -->

# LEDルーレット

<!-- Alright, let's start by building the following application: -->

さて、次のアプリケーションをビルドするところから始めましょう。

<p align="center">
<img src="https://i.imgur.com/0k1r2Lc.gif">
</p>

<!-- 
I'm going to give you a high level API to implement this app but don't worry we'll do low level
stuff later on. The main goal of this chapter is to get familiar with the *flashing* and debugging
process.
 -->

私は、このアプリケーションを実装するための高レベルなAPIを提供します。しかし、心配しないで下さい。低レベルな事項も、後ほど扱います。
この章の主な目的は、*Flashに書き込むこと*とデバッグプロセスに慣れることです。

<!-- 
Throughout this text we'll be using the starter code that's in the [discovery] repository. Make sure
you always have the latest version of the master branch because this website tracks that branch.
 -->

この文書を通して、[discovery]レポジトリにあるスターターコードを使います。
常に手元のコードが、最新バージョンのmasterブランチであることを確かめて下さい。このウェブサイトは、masterブランチに追従しています。

<!-- 
The starter code is in the `src` directory of that repository. Inside that directory there are more
directories named after each chapter of this book. Most of those directories are starter Cargo
projects.
 -->

スターターコードは、discoveryレポジトリの`src`ディレクトリ内にあります。このディレクトリ内には、この本の各章の名前がついたディレクトリがあります。
ほとんどのこれらのディレクトリは、Cargoプロジェクトの開始地点です。

[discovery]: https://github.com/rust-embedded/discovery

<!-- Now, jump into the `src/05-led-roulette` directory. Check the `src/main.rs` file: -->

それでは、`src/05-led-roulette`ディレクトリに飛び込みましょう。`src/main.rs`ファイルを確認して下さい。

``` rust
{{#include src/main.rs}}
```

<!-- 
Microcontroller programs are different from standard programs in two aspects: `#![no_std]` and
`#![no_main]`.
 -->

マイクロコントローラのプログラムは2つの点で通常のプログラムとは異なります。`#![no_std]`と`#![no_main]`です。

<!-- 
The `no_std` attribute says that this program won't use the `std` crate, which assumes an underlying
OS; the program will instead use the `core` crate, a subset of `std` that can run on bare metal
systems (i.e., systems without OS abstractions like files and sockets).
 -->

`no_std`アトリビュートは、プログラムが`std`クレートを使わないことを意味しています。stdクレートはOSが基盤として存在していることを前提としています。
マイクロコントローラのプログラムは、代わりに、`core`クレートを使います。
coreクレートは`std`のサブセットで、ベアメタルシステム（つまり、ファイルやソケットと言ったOSの抽象化なしに動作するシステム）で動作することができます。

<!-- 
The `no_main` attribute says that this program won't use the standard `main` interface, which is
tailored for command line applications that receive arguments. Instead of the standard `main` we'll
use the `entry` attribute from the [`cortex-m-rt`] crate to define a custom entry point. In this
program we have named the entry point "main", but any other name could have been used. The entry
point function must have signature `fn() -> !`; this type indicates that the function can't return
-- this means that the program never terminates.
 -->

`no_main`アトリビュートは、このプログラムが標準の`main`インタフェースを使わないことを意味します。標準の`main`インタフェースは、
引数を受け取るコマンドラインアプリケーション向けに作られています。
カスタムエントリーポイントを定義するために、標準の`main`の代わりに、[`cortex-m-rt`]から`entry`アトリビュートを使います。
このプログラムでは、「main」という名前のエントリーポイントを持ちますが、どのような名前でも使えます。
エントリーポイントの関数は`fn() -> !`のシグネチャを持つ必要があります。このシグネチャ型は、関数が返らないことを示しています。
これは、このプログラムが停止しないことを意味します。

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

<!-- 
If you are a careful observer, you'll also notice there is a `.cargo` directory in the Cargo project
as well. This directory contains a Cargo configuration file (`.cargo/config`) that tweaks the
linking process to tailor the memory layout of the program to the requirements of the target device.
This modified linking process is a requirement of the `cortex-m-rt` crate.
 -->

注意深く観察すると、Cargoプロジェクトに`.cargo`ディレクトリがあることに気づくでしょう。
このディレクトリはCargoの設定ファイル（`.cargo/config`）を含んでいます。この設定ファイルは、
ターゲットデバイスで要求されるプログラムのメモリレイアウトに合わせて、リンクプロセスを微調整します。
この修正が加えられたリンクプロセスは、`cortex-m-rt`クレートで求められます。

<!-- Alright, let's start by building this program. -->

では、プログラムをビルドするところから始めましょう。
