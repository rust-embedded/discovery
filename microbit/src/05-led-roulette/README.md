# LED轮盘

好的，让我们从构建以下应用程序开始：

<p align="center">
<video src="../assets/roulette_fast.mp4" loop autoplay>
</p>

我将为您提供一个高级API来实现这个应用程序，但不要担心我们稍后会做一些低级的事情。本章的主要目标是熟悉*闪烁*和调试过程。

入门代码位于directory存储库`src`的目录中。在该目录中，还有更多以本书每一章命名的目录。这些目录中的大多数都是启动Cargo项目。

现在，跳转到`src/05-led-roulette`目录。检查`src/main.rs`文件：

``` rust
{{#include src/main.rs}}
```

微控制器程序在两个方面不同于标准程序：`#![no_std]`和`#![no_main]`。

该`no_std`属性表示该程序不会使用`std`假定底层操作系统的crate；该程序将改为使用`core` crate，它
的一个子集`std`可以在裸机系统上运行（即，没有OS抽象的系统，如文件和套接字）。

该`no_main`属性表示该程序不会使用标准`main`接口，该接口是为接收参数的命令行应用程序量身定制的。
`main`我们将使用crate中的`entry`属性[`cortex-m-rt`]crate来定义自定义入口点，而不是标准。在这个程
序中，我们将入口点命名为"main"，但也可以使用任何其他名称。入口点函数必须有签名`fn() -> !`；这种类型表示
函数不能返回--这意味着程序永远不会终止。

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

如果你是一个细心的观察者，你也会注意到Cargo项目中有一个`.cargo`目录。该目录包含一个Cargo配置文件
(`.cargo/config`)，它调整链接过程以根据目标设备的要求调整程序的内存布局。这个修改后的链接过
程是`cortex-m-rt`crate的要求。

此外，还有一个`Embed.toml`文件

```toml
{{#include Embed.toml}}
```

该文件提供`cargo-embed`：

* 我们正在使用nrf52833或nrf51822，您将再次必须从正在使用的芯片中删除注释，就像您在第3章中所做的那样。
* 我们希望在闪存之后停止芯片，这样我们的程序就不会立即跳转到循环
* 我们想禁用RTT，RTT是一种允许芯片向调试器发送文本的协议。您实际上已经看到了RTT的实际应用，它是在第3章中发送"Hello World"的协议。
* 我们要启用GDB，这将是调试过程所必需的

好的，让我们从构建这个程序开始。
