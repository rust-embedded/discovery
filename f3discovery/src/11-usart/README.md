# USART

微控制器有一个叫做USART的外设，它代表通用同步/异步接收机/发射机。该外围设备可以被配置为与多个通信协议（如串行通信协议）一起工作。

在本章中，我们将使用串行通信在微控制器和计算机之间交换信息。但在我们这样做之前，我们必须把所有的东西都连接起来。

我之前提到过，该协议涉及两条数据线：TX和RX。TX代表发射机，RX代表接收机。发射器和接收器是相对的术语；
哪一条线路是发射机，哪一条线是接收机，取决于你从通信的哪一侧看线路。

### 更新的修订

如果您有较新版本的主板，并且使用板USB <-> 串行功能，则`auxiliary` crate 将引脚`PC4`设置为TX线，
引脚`PC5`设置为RX线。

所有的东西都已经在板上布线了，所以你不需要自己布线。您可以转到[下一节](send-a-single-byte.html)。

### 较旧的电路板版本 / 外部串行模块

如果您使用的是外部USB <-> 串行模块，则**需要**在`Cargo.toml`中启用`aux11` crate依赖项的`adapter`功能。

``` toml
[dependencies.aux11]
path = "auxiliary"
# enable this if you are going to use an external serial adapter
features = ["adapter"] # <- uncomment this
```

我们将使用引脚`PA9`作为微控制器的TX线，`PA10`作为其RX线。换句话说，引脚`PA9`将数据输出到其导线上，
而引脚`PA10`监听其导线上的数据。

我们可以使用一对不同的引脚作为TX和RX引脚。[数据表]第44页有一个表格， 列出了我们可能使用的所有其他引脚。

[Data Sheet]: http://www.st.com/resource/en/datasheet/stm32f303vc.pdf

串行模块还具有TX和RX引脚。我们将不得不*交叉*这些引脚：即将微控制器的TX引脚连接到串行模块的RX引脚，
将微型计算机的RX引脚连接到串口模块的TX引脚。下图显示了所有必要的连接。

<p>
<img height=640 title="F3 <-> Serial connection" src="../assets/f3-serial.png">
</p>

以下是连接微控制器和串行模块的建议步骤：

- 关闭OpenOCD和`itmdump`。
- 断开F3和串行模块的USB电缆。
- 使用母对公（F/M）导线将F3 GND引脚之一连接到串行模块的GND引脚。最好是黑色的。
- 使用F/M导线将F3背面的PA9引脚连接到串行模块的RXI引脚。
- 使用F/M导线将F3背面的PA10引脚连接到串行模块的TXO引脚。
- 现在将USB电缆连接到F3。
- 最后将USB电缆连接到串行模块。
- 重新启动OpenOCD和`itmdump`。

一切都准备好了！让我们继续来回发送数据。
