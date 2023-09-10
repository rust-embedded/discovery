# Discovery

> 通过[Rust]探索微控制器的世界！

[Rust]: https://www.rust-lang.org/

这本书是关于基于微控制器的嵌入式系统的入门课程，它使用Rust作为教学语言，而不是通常的C/C++。

## 范围

将涵盖以下主题（最终，我希望）：

- 如何编写、构建、刷新和调试"嵌入式"(Rust)程序。

- 微控制器中常见的功能（"外部设备"）：数字输入和输出、脉冲宽度调制 (PWM)、模数转换器 (ADC)、串行、I2C和SPI等常见通信协议。

- 多任务处理概念：协作与抢占式多任务处理、中断、调度程序等。

- 控制系统概念：传感器、校准、数字滤波器、执行器、开环控制、闭环控制等。

## 方法

- 初学者友好。无需具备微控制器或嵌入式系统方面的经验。

- 动手。大量的练习将理论付诸实践。您将在这里完成大部分工作。 您将在这里完成大部分工作。

- 工具居中。我们将大量使用工具来简化开发。"真正的"调试、使用GDB和日志记录将在早期引入。在这里使用LED作为调试机制是不合适的。

## 非目标

本书范围之外的内容：

- 教Rust。已经有很多关于该主题的材料。我们将专注于微控制器和嵌入式系统。

- 是一本关于电路理论或电子学的综合性书籍。我们将仅介绍了解某些设备如何工作所需的最低要求。

- 涵盖链接描述文件和引导过程等细节。例如，我们将使用现有工具帮助您将代码放到板上，但不会详细介绍这些工具的工作原理。

另外我不打算将这个材料移植到其他开发板上；本书将独家使用STM32F3DISCOVERY开发板。

## 报告问题

这本书的源代码在[这个存储库]。如果您遇到任何错字或代码问题，请在[问题跟踪器]上报告。

[这个存储库]: https://github.com/rust-embedded/discovery
[问题跟踪器]: https://github.com/rust-embedded/discovery/issues

## 其他嵌入式Rust资源

这本Discovery书只是[嵌入式工作组]提供的几个嵌入式Rust资源之一。 完整的选择可以在[The Embedded Rust Bookshelf]中找到。
这包括[常见问题]列表。

[嵌入式工作组]: https://github.com/rust-embedded/wg
[The Embedded Rust Bookshelf]: https://docs.rust-embedded.org
[常见问题]: https://docs.rust-embedded.org/faq.html

## 赞助

<p>
<a href="http://integer32.com/">
<img style="width: 50%" title="integer 32" src="assets/integer32.svg">
</a>
</p>

非常感谢[integer 32](http://integer32.com/)赞助我写这本书！请给他们很多工作（他们做Rust咨询！）
所以他们别无选择，只能雇佣更多Rustaceans <3。
