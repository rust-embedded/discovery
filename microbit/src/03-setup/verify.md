# 验证安装

让我们验证所有工具是否已正确安装。

## 仅限Linux

### 验证权限

使用USB数据线将micro:bit连接到您的计算机。

micro:bit现在应该以USB设备（文件）的形式出现在`/dev/bus/usb`。让我们看看它是如何被枚举的：

``` console
$ lsusb | grep -i "NXP ARM mbed"
Bus 001 Device 065: ID 0d28:0204 NXP ARM mbed
$ # ^^^        ^^^
```

在我的例子中，micro:bit连接到总线#1被枚举为设备#65。这意味着该文件`/dev/bus/usb/001/065`*是*micro:bit。
让我们检查它的权限：

``` console
$ ls -l /dev/bus/usb/001/065
crw-rw-rw-. 1 root root 189, 64 Sep  5 14:27 /dev/bus/usb/001/065
```

权限应该是`crw-rw-rw-`。如果不是...然后检查您的[udev规则]并尝试重新加载它们：

[udev规则]: linux.md#udev-rules

``` console
$ sudo udevadm control --reload-rules
```

# 全部

## 验证cargo-embed
首先，使用USB数据线将micro:bit连接到您的计算机。

micro:bit的USB端口旁边至少有一个橙色LED应该亮起。此外，如果您从未在micro:bit上刷过其他程序，
则micro:bit附带的默认程序应该开始闪烁其背面的红色LED，您可以忽略它们。

接下来，您将不得不在本书的源代码`src/03-setup`目录中进行修改`Embed.toml`。
在该`default.general`部分中，您将找到两个已注释掉的芯片的变量：

```toml
[default.general]
# chip = "nrf52833_xxAA" # uncomment this line for micro:bit V2
# chip = "nrf51822_xxAA" # uncomment this line for micro:bit V1
```

如果您使用的是micro:bit v2板，请取消注释第一行，对于v1，请取消注释第二行。

接下来运行以下命令之一：

```
$ # make sure you are in src/03-setup of the books source code
$ # If you are working with micro:bit v2
$ rustup target add thumbv7em-none-eabihf
$ cargo embed --target thumbv7em-none-eabihf

$ # If you are working with micro:bit v1
$ rustup target add thumbv6m-none-eabi
$ cargo embed --target thumbv6m-none-eabi
```

如果一切正常，cargo-embed应该首先编译这个目录中的小示例程序，然后刷新它，
最后打开一个漂亮的基于文本的用户界面，打印Hello World。

(如果没有，请查看[一般故障排除]说明。)

[一般故障排除]: ../appendix/1-general-troubleshooting/index.html

此输出来自您刚刚嵌入micro:bit的小型Rust程序。一切正常，您可以继续下一章！
