<!-- # Build it -->

# ビルド

<!-- 
The first step is to build our "binary" crate. Because the microcontroller has a different
architecture than your laptop we'll have to cross compile. Cross compiling in Rust land is as simple
as passing an extra `--target` flag to `rustc`or Cargo. The complicated part is figuring out the
argument of that flag: the *name* of the target.
 -->

最初の一歩は、「バイナリ」クレートをビルドすることです。マイクロコントローラは、あなたのノートPCとアーキテクチャが異なるため、クロスコンパイルする必要が有ります。
Rustでのクロスコンパイルは、`rustc`かCargoに追加の`--target`フラグを渡すだけです。
複雑な部分は、フラグの引数、つまりターゲットの*名前*を見つけ出すことです。

<!-- 
The microcontroller in the F3 has a Cortex-M4F processor in it. `rustc` knows how to cross compile
to the Cortex-M architecture and provides 4 different targets that cover the different processor
families within that architecture:
 -->

F3のマイクロコントローラは、Cortex-M4Fプロセッサを搭載しています。
`rustc`は、Cortex-Mアーキテクチャ向けにクロスコンパイルする方法を知っており、4つの異なるターゲットを提供しています。
これらのターゲットは、Cortex-Mアーキテクチャの異なるプロセッサファミリを対象としています。

<!-- 
- `thumbv6m-none-eabi`, for the Cortex-M0 and Cortex-M1 processors
- `thumbv7m-none-eabi`, for the Cortex-M3 processor
- `thumbv7em-none-eabi`, for the Cortex-M4 and Cortex-M7 processors
- `thumbv7em-none-eabihf`, for the Cortex-M4**F** and Cortex-M7**F** processors
 -->

- `thumbv6m-none-eabi`, Cortex-M0とCortex-M1プロセッサ向け
- `thumbv7m-none-eabi`, Cortex-M3プロセッサ向け
- `thumbv7em-none-eabi`, Cortex-M4とCortex-M7プロセッサ向け
- `thumbv7em-none-eabihf`, Cortex-M4**F**とCortex-M7**F**プロセッサ向け

<!-- 
For the F3, we'll to use the `thumbv7em-none-eabihf` target. Before cross compiling you have to
download pre-compiled version of the standard library (a reduced version of it actually) for your
target. That's done using `rustup`:
 -->

F3に対しては、`thumbv7em-none-eabihf`ターゲットを使います。
クロスコンパイルする前に、あらかじめターゲット向けにコンパイルされた標準ライブラリ（実際にはその縮小版）をダウンロードしなければなりません。
`rustup`を使ってダウンロードできます。

``` console
$ rustup target add thumbv7em-none-eabihf
```

<!-- 
You only need to do the above step once; `rustup` will re-install a new standard library
(`rust-std` component) whenever you update your toolchain.
 -->

上記のコマンドは一度だけ実行する必要が有ります。`rustup`は、ツールチェインをアップデートするときに、
新しい標準ライブラリ（`rust-std`コンポーネント）を再インストールしてくれます。

<!-- 
With the `rust-std` component in place you can now cross compile the program using Cargo:
 -->

`rust-std`コンポーネントがあれば、Cargoを使ってプログラムをクロスコンパイルできます。

``` console
$ # `src/05-led-roulette`ディレクトリにいることを確認して下さい
$ cargo build --target thumbv7em-none-eabihf
   Compiling semver-parser v0.7.0
   Compiling aligned v0.1.1
   Compiling libc v0.2.35
   Compiling bare-metal v0.1.1
   Compiling cast v0.2.2
   Compiling cortex-m v0.4.3
   (..)
   Compiling stm32f30x v0.6.0
   Compiling stm32f30x-hal v0.1.2
   Compiling aux5 v0.1.0 (file://$PWD/aux)
   Compiling led-roulette v0.1.0 (file://$PWD)
    Finished dev [unoptimized + debuginfo] target(s) in 35.84 secs
```

<!-- 
> **NOTE** Be sure to compile this crate *without* optimizations. The provided Cargo.toml file and build command above will ensure optimizations are off. 
 -->

> **注記** このクレートを必ず最適化*なしで*コンパイルして下さい。提供しているCargo.tomlと上記のビルドコマンドは、最適化をオフにしています。

<!-- 
OK, now we have produced an executable. This executable won't blink any leds, it's just a simplified version that we will build upon later in the chapter. As a sanity check, let's verify that the produced executable is actually an ARM binary:
 -->

OK、これで実行ファイルが作成されました。この実行ファイルは、LEDを点滅させません。これは、この章の後半でビルドする単純なものです。
正当性検査のため、作成した実行ファイルが本当にARMのバイナリかどうか確認してみましょう。

``` console
$ # `readelf -h target/thumbv7em-none-eabihf/debug/led-roulette`と同じです
$ cargo readobj --target thumbv7em-none-eabihf --bin led-roulette -- -file-headers
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0x0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x8000197
  Start of program headers:          52 (bytes into file)
  Start of section headers:          740788 (bytes into file)
  Flags:                             0x5000400
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         2
  Size of section headers:           40 (bytes)
  Number of section headers:         20
  Section header string table index: 18
```

<!-- Next, we'll flash the program into our microcontroller. -->

次は、このプログラムをマイクロコントローラのFlashに書き込みます。