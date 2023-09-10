# 再次 Loopback

将计算机与蓝牙模块配对后，操作系统应为您创建了设备文件/COM端口。
在Linux上，它应该是`/dev/rfcomm*`；在mac上，它应该是`/dev/cu.*`;在Windows上，它应该是一个新的COM端口。

我们现在可以用minicom/PuTTY测试蓝牙模块。
由于该模块不像串行模块那样具有用于发送和接收事件的LED指示灯， 因此我们将使用loopback连接测试该模块：

<p>
<img height=640 title="F3 <-> Bluetooth connection (loopback)" src="../assets/f3-bluetooth-loopback.png">
</p>

只需使用F/F线将模块的TXD引脚连接到RXD引脚。

现在，使用`minicom`/`PuTTY`连接到设备：

``` console
$ minicom -D /dev/rfcomm0
```

连接后，蓝牙模块的闪烁模式应更改为：长暂停，然后快速闪烁两次。

在minicom/PuTTY终端内输入应该会回复您输入的内容。
