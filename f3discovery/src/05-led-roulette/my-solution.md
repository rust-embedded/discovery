<!-- # My solution -->

# 解答例

<!-- What solution did you come up with? -->

あなたの解答は、どのようになりましたか？

<!-- Here's mine: -->

私の解答は、次の通りです。

``` rust
{{#include examples/my-solution.rs}}
```

<!-- One more thing! Check that your solution also works when compiled in "release" mode: -->

もうひとつ！あなたの解答が「release」モードでコンパイルしても動作するか、確認して下さい。

``` console
$ cargo build --target thumbv7em-none-eabihf --release
```

<!-- You can test it with this `gdb` command: -->

次の`gdb`コマンドでテスト可能です。

``` console
$ # or, you could simply call `cargo run --target thumbv7em-none-eabihf --release`
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/led-roulette
$ #                                              ~~~~~~~
```

<!-- 
Binary size is something we should always keep an eye on! How big is your solution? You can check
that using the `size` command on the release binary:
 -->

バイナリサイズは、常に注意を払う必要があります！あなたの解答では、どの程度の大きさになりましたか？
リリースバイナリに`size`コマンドを使うことで、確認できます。

``` console
$ # size target/thumbv7em-none-eabihf/debug/led-rouletteと等価です
$ cargo size --target thumbv7em-none-eabihf --bin led-roulette -- -A
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
led-roulette  :
section               size        addr
.vector_table          404   0x8000000
.text                21144   0x8000194
.rodata               3144   0x800542c
.data                    0  0x20000000
.bss                     4  0x20000000
.uninit                  0  0x20000004
.debug_abbrev        19160         0x0
.debug_info         471239         0x0
.debug_aranges       18376         0x0
.debug_ranges       102536         0x0
.debug_str          508618         0x0
.debug_pubnames      76975         0x0
.debug_pubtypes     112797         0x0
.ARM.attributes         58         0x0
.debug_frame         55848         0x0
.debug_line         282067         0x0
.debug_loc             845         0x0
.comment               147         0x0
Total              1673362


$ cargo size --target thumbv7em-none-eabihf --bin led-roulette --release -- -A
    Finished release [optimized + debuginfo] target(s) in 0.03s
led-roulette  :
section              size        addr
.vector_table         404   0x8000000
.text                5380   0x8000194
.rodata               564   0x8001698
.data                   0  0x20000000
.bss                    4  0x20000000
.uninit                 0  0x20000004
.debug_loc           9994         0x0
.debug_abbrev        1821         0x0
.debug_info         74974         0x0
.debug_aranges        600         0x0
.debug_ranges        6848         0x0
.debug_str          52828         0x0
.debug_pubnames     20821         0x0
.debug_pubtypes     18891         0x0
.ARM.attributes        58         0x0
.debug_frame         1088         0x0
.debug_line         15307         0x0
.comment               19         0x0
Total              209601
```

<!-- > **NOTE** The Cargo project is already configured to build the release binary using LTO. -->

> **注記** このCargoプロジェクトは、LTOを使ってリリースバイナリをビルドするように設定されています。

<!-- 
Know how to read this output? The `text` section contains the program instructions. It's around 2KB
in my case. On the other hand, the `data` and `bss` sections contain variables statically allocated
in RAM (`static` variables). A `static` variable is being used in `aux5::init`; that's why it shows 4
bytes of `bss`.
 -->

この出力をどう読めばよいか知っていますか？`text`セクションは、プログラムの命令を含んでいます。私の場合、約2KBです。
一方、`data`と`bss`セクションは、RAMに静的に割り当てられた変数（`static`変数）を含みます。
`aux5::init`で`static`変数を1つ使っています。そのため、`bss`のサイズは4バイトとなっています。

<!-- 
One final thing! We have been running our programs from within GDB but our programs don't depend on
GDB at all. You can confirm this be closing both GDB and OpenOCD and then resetting the board by
pressing the black button on the board. The LED roulette application will run without intervention
of GDB.
 -->

最後にもうひとつ！プログラムをGDB内で実行していますが、プログラムはGDBに全く依存していません。
GDBとOpenOCDを両方とも終了して、ボード上の黒いボタンを押してボードをリセットすることで、このことを確認できます。
LEDルーレットアプリケーションは、GDBの介入なしに動作します。
