# \*nix 工具

## 连接micro:bit板

如果您将micro:bit板连接到您的计算机，您应该会看到一个新的TTY设备出现在`/dev`。

``` console
$ # Linux
$ dmesg | tail | grep -i tty
[63712.446286] cdc_acm 1-1.7:1.1: ttyACM0: USB ACM device
```

这是USB<->串行设备。在 Linux 上，它被命名为`tty*` (通常是`ttyACM*` 或 `ttyUSB*`)。
在Mac OS`ls /dev/cu.usbmodem*`上将显示串行设备。

但`ttyACM0`究竟是什么？当然是文件！一切都是\*nix中的文件：

```
$ ls -l /dev/ttyACM0
crw-rw----. 1 root plugdev 166, 0 Jan 21 11:56 /dev/ttyACM0
```

您可以通过简单地写入此文件来发送数据：

``` console
$ echo 'Hello, world!' > /dev/ttyACM0
```

每当您输入此命令时，您应该会看到micro:bit上的橙色LED，就在USB端口旁边，闪烁片刻。

## minicom

我们将使用程序`minicom`使用键盘与串行设备交互。

我们必须先配置`minicom`然后才能使用它。有很多方法可以做到这一点，但我们将使用`.minirc.dfl`主目录中的文件。创建一个包含
创建一个包含`~/.minirc.dfl`文件，包含以下内容：

``` console
$ cat ~/.minirc.dfl
pu baudrate 115200
pu bits 8
pu parity N
pu stopbits 1
pu rtscts No
pu xonxoff No
```

> **注意**：确保此文件以换行符结尾！否则，`minicom`将无法读取它。

该文件应该易于阅读（最后两行除外），但让我们逐行查看：

- `pu baudrate 115200`。将波特率设置为115200bps。
- `pu bits 8`。每帧8位。
- `pu parity N`。无相同校验。
- `pu stopbits 1`。1个stop bit。
- `pu rtscts No`。没有硬件控制流
- `pu xonxoff No`。没有软件控制流程。

一旦这一切就绪，我们就可以启动`minicom`。

``` console
$ # NOTE you may need to use a different device here
$ minicom -D /dev/ttyACM0 -b 115200
```

这通过`minicom`在`/dev/ttyACM0`打开串行设备，并将其波特率设置为115200。将弹出基于文本的用户界面（TUI）。

<p>
<img title="minicom" src="../assets/minicom.png">
</p>

您现在可以使用键盘发送数据！请输入一些内容。请注意，文本UI不会回显您键入的内容。但是，如果你
注意micro:bit顶部的黄色LED，你会注意到每当你键入某个内容时，它都会闪烁。

## `minicom`命令

`minicom`通过键盘快捷键公开命令。在Linux上，快捷方式以`Ctrl+A`开头。 在Mac上，快捷键以`Meta`键开头。
以下是一些有用的命令：

- `Ctrl+A` + `Z`。Minicom 命令摘要
- `Ctrl+A` + `C`。清除屏幕
- `Ctrl+A` + `X`。退出并重置
- `Ctrl+A` + `Q`。退出并重置

> **注意**：Mac用户：在上述命令中，将`Ctrl+A`替换为`Meta`。
