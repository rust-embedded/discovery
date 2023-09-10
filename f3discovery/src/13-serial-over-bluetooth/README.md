# 蓝牙串行

现在，我们验证了蓝牙模块与minicom/PuTTY一起工作，让我们将其连接到微控制器：

<p>
<img height=640 title="F3 <-> Bluetooth connection" src="../assets/f3-bluetooth.png">
</p>

建议的连接步骤：

- 关闭OpenOCD和`itmdump`。
- 断开F3与计算机的连接。
- 使用母对母（F/F）导线（最好是黑色）将F3的GND引脚连接到模块的GND插针。
- 使用F/F线（最好是红色线）将F3的5V引脚连接到模块的VCC引脚。
- 使用F/F线将F3背面的PA9（TX）引脚连接到蓝牙的RXD引脚。
- 使用F/F线将F3背面的PA10（RX）引脚连接到蓝牙的TXD引脚。
- 现在，使用USB电缆连接F3和计算机。
- 重新启动OpenOCD和`itmdump`。

就这样！你应该能够运行你在第[11章节]中编写的所有程序，而无需修改！只需确保打开了正确的串行设备/COM端口。

**注意**：如果您在与蓝牙设备通信时遇到问题，您可能需要使用较低的波特率初始化USART1。
如[此代码段](https://github.com/rust-embedded/discovery/blob/master/f3discovery/src/11-usart/auxiliary/src/lib.rs#L31)所述，
将其从115200 bps降至9600 bps可能会有所帮助

[11章节]: ../11-usart/index.html
