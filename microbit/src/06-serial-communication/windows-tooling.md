# Windows 工具

首先拔掉你的micro:bit。

在插入micro:bit之前，在终端上运行以下命令：

``` console
$ mode
```

它将打印连接到您的计算机的设备列表。以`COM`的名字开头的是串行设备。这是我们将要使用的设备。
在插入串行模块*之前*请注意所有`COM`*ports*`mode`输出。

现在，插入micro:bit并再次运行命令`mode`。如果您看到`COM`列表中出现了一个新端口，那么您就有了分
配给micro:bit串行功能的COM端口。

现在启动`putty`。将弹出一个GUI。

<p>
<img title="PuTTY settings" src="../assets/putty-settings.png">
</p>

在启动屏幕上，应该打开"会话"类别，选择"串行"作为"连接类型"。在"串行线路"字段中输入上一步中获得的`COM`设备，例如`COM3`。

接下来，从左侧菜单中选择"连接/串行"类别。在这个新视图上，确保串行端口配置如下：

- "Speed (baud)": 115200
- "Data bits": 8
- "Stop bits": 1
- "Parity": None
- "Flow control": None

最后，单击打开按钮。现在将显示一个控制台：

<p>
<img title="PuTTY console" src="../assets/putty-console.png">
</p>

如果您在此控制台上键入，micro:bit顶部的黄色LED将闪烁。每次按键应使LED闪烁一次。请注意，控制台
不会回显您键入的内容，因此屏幕将保持空白。
