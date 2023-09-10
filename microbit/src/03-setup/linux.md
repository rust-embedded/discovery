# Linux

以下是一些 Linux 发行版的安装命令。

## Ubuntu 20.04 或最新版本 / Debian 10 或最新版本

> **注意** `gdb-multiarch`是您将用于调试ARM Cortex-M程序的GDB命令
``` console
$ sudo apt-get install \
  gdb-multiarch \
  minicom
```

## Fedora 32 或最新版本
> **注意** `gdb`是您将用于调试 ARM Cortex-M程序的GDB命令
> Cortex-M programs
``` console
$ sudo dnf install \
  gdb \
  minicom
```

## Arch Linux

> **注意** `arm-none-eabi-gdb`是您将用于调试ARM Cortex-M程序的GDB命令
``` console
$ sudo pacman -S \
  arm-none-eabi-gdb \
  minicom
```

## 其他发行版

> **注意** `arm-none-eabi-gdb`是您将用于调试ARM Cortex-M程序的GDB命令

对于没有[ARM预构建工具链](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)软件包的发行版，
请下载"Linux 64-bit"文件并将其`bin`目录放在您的路径中。
这是一种方法：

``` console
$ mkdir -p ~/local && cd ~/local
$ tar xjf /path/to/downloaded/file/gcc-arm-none-eabi-9-2020-q2-update-x86_64-linux.tar.bz2
```

然后，使用您选择的编辑器在适当的shell初始化文件(例如 `~/.zshrc` 或 `~/.bashrc`)中附加到`PATH`：

```
PATH=$PATH:$HOME/local/gcc-arm-none-eabi-9-2020-q2-update/bin
```

## udev 规则

这些规则让您可以在没有root权限的情况下使用像micro:bit这样的USB设备，即`sudo`。

`/etc/udev/rules.d`使用如下所示的内容创建此文件。

``` console
$ cat /etc/udev/rules.d/99-microbit.rules
```

``` text
# CMSIS-DAP for microbit
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
```

然后使用以下命令重新加载udev规则：

``` console
$ sudo udevadm control --reload-rules
```

如果您的计算机上插入了任何板，请拔下它们，然后重新插入。

现在，转到[下一节]。

[下一节]: verify.md
