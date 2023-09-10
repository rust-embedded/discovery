# \*nix 工具

## discovery开发板的更新版本

对于较新的版本，如果您将板连接到计算机，则会在`/dev`中看到一个新的TTY设备。

``` console
$ # Linux
$ dmesg | tail | grep -i tty
[13560.675310] cdc_acm 1-1.1:1.2: ttyACM0: USB ACM device
```

这是USB <-> 串行设备。在Linux上，它被命名为`tty*` (通常是`ttyACM*`或`ttyUSB*`)。

如果你没有看到设备出现，那么你可能有一个较旧版本的电路板；检查下一节，其中包含旧版本的说明。
如果您有更新的版本， 请跳过下一节，转到"minicom"部分。

## discovery板 / 外部串行模块的旧版本

将串行模块连接到计算机，让我们找出分配给它的操作系统名称。

> **注意**：在Mac上，USB设备的名称如下`/dev/cu.usbserial-*`。您不会使用`dmesg`找到它，
> 而是使用`ls -l /dev | grep cu.usb`并相应地调整以下命令！

``` console
$ dmesg | grep -i tty
(..)
[  +0.000155] usb 3-2: FTDI USB Serial Device converter now attached to ttyUSB0
```

但`ttyUSB0`是什么？这当然是一份文件！所有内容都是\*nix中的文件：

``` console
$ ls -l /dev/ttyUSB0
crw-rw-rw- 1 root uucp 188, 0 Oct 27 00:00 /dev/ttyUSB0
```

> **注意**：如果以上权限为`crw-rw----`，则udev规则未正确设置，请参阅[udev规则](../03-setup/linux.html#udev-rules)

您只需向以下文件写入即可发送数据：

``` console
$ echo 'Hello, world!' > /dev/ttyUSB0
```

您应该看到串行模块上的TX (红色) LED 闪烁一次，而且非常快！

## 所有版本：minicom

使用`echo`处理串行设备远非符合人体工程学。因此，我们将使用程序`minicom`使用键盘与串行设备进行交互。

我们必须在使用`minicom`之前配置它。有很多方法可以做到这一点，但我们将使用`.minirc.dfl`文件。
在`~/.minirc.dfl`中创建包含以下内容的文件：

``` console
$ cat ~/.minirc.dfl
pu baudrate 115200
pu bits 8
pu parity N
pu stopbits 1
pu rtscts No
pu xonxoff No
```

> **注意**：请确保此文件以换行结尾！否则，`minicom`将无法读取它。

该文件应该易于阅读（除了最后两行），但让我们逐行阅读：

- `pu baudrate 115200`. 将波特率设置为 115200 bps。
- `pu bits 8`. 每帧 8 bits。
- `pu parity N`. 无对等校验。
- `pu stopbits 1`. 1 stop bit。
- `pu rtscts No`. 无软件控制流程。
- `pu xonxoff No`. 无软件控制流程。

一旦到位，我们就可以启动`minicom`.

``` console
$ # NOTE you may need to use a different device here
$ minicom -D /dev/ttyACM0 -b 115200
```

这告诉`minicom`在`/dev/ttyACM0`打开串行设备，并将其波特率设置为115200。将弹出一个基于文本的用户界面（TUI）。

<p>
<img title="minicom" src="../assets/minicom.png">
</p>

您现在可以使用键盘发送数据了！继续输入一些内容。请注意，TUI*不会*回复您输入的内容，但是，如果您
使用的是外部模块，您*可能*会看到模块上的一些LED随着每次按键而闪烁。

## `minicom` 命令

`minicom`通过键盘快捷键公开命令。在Linux上，快捷键以`Ctrl+A`开头。在mac上，快捷键以`Meta`键开头。下面是一些有用的命令：

- `Ctrl+A` + `Z`. Minicom 命令摘要
- `Ctrl+A` + `C`. 清除屏幕
- `Ctrl+A` + `X`. 退出并重置
- `Ctrl+A` + `Q`. 退出但不重置

> **注意**：mac用户在上述命令中，将`Ctrl+A`替换为`Meta`。
