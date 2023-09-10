# 蓝牙设置

是时候去掉一些电线了。串行通信不仅可以在USB协议之上进行仿真；它也可以在蓝牙协议之上进行仿真。这种串行蓝牙协议被称为RFCOMM。

在我们将蓝牙模块与微控制器一起使用之前，让我们先使用minicom/PuTTY与它进行交互。

我们需要做的第一件事是：打开蓝牙模块。我们必须使用以下连接共享F3电源：

<p>
<img height=640 title="F3 <-> Bluetooth connection (power only)" src="../assets/f3-bluetooth-power-only.png">
</p>

建议的连接步骤如下：

- 关闭OpenOCD和`itmdump`。
- 断开F3和串行模块的USB电缆。
- 使用母对母 (F/F) 导线将F3的GND引脚连接到蓝牙的GND插针。最好是黑色的。
- 使用F/F线将F3的5V引脚连接到蓝牙的VCC引脚。最好是红色的。
- 然后，将USB电缆连接回F3。
- 重新启动OpenOCD和`itmdump`。

打开F3板电源后，蓝牙模块上的两个LED（蓝色和红色）应立即开始闪烁。

接下来要做的是将计算机和蓝牙模块配对。AFAIK，Windows和mac用户只需使用其操作系统默认蓝牙管理器即可进行配对。蓝牙模块默认引脚为1234

Linux用户将必须遵循（部分）[这些说明]。

[这些说明]: linux.md
