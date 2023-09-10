# LED轮盘

好了，让我们从构建以下应用程序开始：

<p>
<img src="https://i.imgur.com/0k1r2Lc.gif">
</p>

我会给你一个高级API来实现这个应用程序，但别担心我们稍后会做低级的工作。本章的主要目标是熟悉*闪烁*和调试过程。

在本文中，我们将使用[discovery]存储库中的起始代码。确保您始终拥有主分支的最新版本，因为此网站跟踪该分支。

起始代码位于该存储库的`src`目录中。在该目录中，有更多以本书每一章命名的目录。这些目录中的大多数都是初级货运项目。

[discovery]: https://github.com/rust-embedded/discovery

现在，跳转到`src/05-led-roulette`目录。检查`src/main.rs`文件：

``` rust
{{#include src/main.rs}}
```

微控制器程序在两个方面与标准程序不同：`#![no_std]`和`#![no_main]`。

`no_std`属性表示该程序不会使用`std`，该crate假定有一个底层操作系统；该程序将转而使用`core` crate，这是
`std`的一个子集，可以在裸机系统上运行（文件和套接字等OS抽象的系统）。

`no_main`属性表示该程序不会使用标准`main`接口，它是为接收参数的命令行应用程序定制的。我们将使用[`cortex-m-rt`] crate中
的`entry`属性来定义自定义入口点，而不是标准的"main"在这个程序中，我们将入口点命名为"main"，但可以使用任何其他名称。
入口点函数必须具有签名`fn() -> !`这种类型表示函数不能返回&这意味着程序永远不会终止。

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

如果你是一个细心的观察者，你也会注意到Cargo项目中有一个`.cargo`目录。该目录包含一个Cargo配置文件(`.cargo/config`)，
该文件调整链接过程，以根据目标设备的要求调整程序的内存布局。这种改进的连接过程是`cortex-m-rt` crate的要求。
在以后的章节中，您还将进一步调整`.cargo/config`，以使构建和调试更容易。 

好了，让我们开始构建这个程序。
