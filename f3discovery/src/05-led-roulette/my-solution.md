# 我的解决方案

你想出了什么解决方案？

这是我的：

``` rust
{{#include examples/my-solution.rs}}
```

还有一件事！检查您的解决方案在"release"模式下编译时是否也能正常工作：

``` console
$ cargo build --target thumbv7em-none-eabihf --release
```

您可以使用以下`gdb`命令进行测试：

``` console
$ # or, you could simply call `cargo run --target thumbv7em-none-eabihf --release`
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/led-roulette
$ #                                              ~~~~~~~
```

二进制大小是我们应该时刻关注的事情！您的解决方案有多大？您可以在发布二进制文件上使用`size`命令进行检查：

``` console
$ # equivalent to size target/thumbv7em-none-eabihf/debug/led-roulette
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

> **注意**：Cargo项目已经配置为使用LTO构建发布二进制文件。

知道如何读取此输出吗？`text`部分包含程序说明。 我的情况是大约5.25KB。另一方面`data`和`bss`部分包含
静态分配在RAM中的变量 (`static`变量)。`aux5::init`中使用了一个`static`变量；这就是它显示4字节`bss`的原因。

最后一件事！我们一直在GDB内部运行我们的程序，但我们的程序根本不依赖GDB。
您可以通过关闭GDB和OpenOCD，然后按下板上的黑色按钮重置板来确认这一点。
LED轮盘应用程序将在没有GDB干预的情况下运行。
