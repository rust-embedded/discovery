# Linux

以下是一些Linux发行版的安装命令。

## 必需的程序包

### Ubuntu 18.04 或更新版本 / Debian stretch 或更新版本

> **注意**：`gdb-multiarch`是用于调试ARM Cortex-M程序的GDB命令

<!-- Debian stretch -->
<!-- GDB 7.12 -->
<!-- OpenOCD 0.9.0 -->

<!-- Ubuntu 18.04 -->
<!-- GDB 8.1 -->
<!-- OpenOCD 0.10.0 -->

``` console
sudo apt-get install \
  gdb-multiarch \
  minicom \
  openocd
```

### Ubuntu 14.04 和 16.04

> **注意**：`arm-none-eabi-gdb`是用于调试ARM Cortex-M程序的GDB命令

<!-- Ubuntu 14.04 -->
<!-- GDB 7.6 -->
<!-- OpenOCD 0.7.0 -->

``` console
sudo apt-get install \
  gdb-arm-none-eabi \
  minicom \
  openocd
```

### Fedora 23 或更高版本

``` console
sudo dnf install \
  minicom \
  openocd \
  gdb
```

### Arch Linux

> **注意**：`arm-none-eabi-gdb`是用于调试ARM Cortex-M程序的GDB命令

``` console
sudo pacman -S \
  arm-none-eabi-gdb \
  minicom \
  openocd
```

### 其他发行版

> **注意**：`arm-none-eabi-gdb`是用于调试ARM Cortex-M程序的GDB命令

对于没有[ARM预构建工具链包](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)的发行版，请下载
"Linux 64-bit"文件并将其`bin`目录放在您的路径上。这里有一种方法：

``` console
mkdir -p ~/local && cd ~/local
```
``` console
tar xjf /path/to/downloaded/file/gcc-arm-none-eabi-10-2020-q4-major-x86_64-linux.tar.bz2
```

然后，使用您选择的编辑器在适当的shell init文件中附加到`PATH`(例如`~/.zshrc` 或 `~/.bashrc`):

```
PATH=$PATH:$HOME/local/gcc-arm-none-eabi-10-2020-q4-major-x86_64-linux/bin
```

## 可选软件包

### Ubuntu / Debian

``` console
sudo apt-get install \
  bluez \
  rfkill
```

### Fedora

``` console
sudo dnf install \
  bluez \
  rfkill
```

### Arch Linux

``` console
sudo pacman -S \
  bluez \
  bluez-utils \
  rfkill
```

## udev规则

这些规则允许您在没有root权限即`sudo`的情况下使用F3和串行模块等USB设备。

创建`99-openocd.rules`在`/etc/udev/rules.d`使用`lsusb`输出中的`idVendor`和`idProduct`。

例如，使用USB电缆将STM32F3DISCOVERYY连接到计算机。确保将电缆连接到"USB ST-LINK"端口，即板边缘中央的USB端口。

执行 `lsusb`:
``` console
lsusb | grep ST-LINK
```
结果应该是：
```
$ lsusb | grep ST-LINK
Bus 003 Device 003: ID 0483:374b STMicroelectronics ST-LINK/V2.1
```
因此`idVendor`为`0483`和`idProduct`为`374b`。

### 创建 `/etc/udev/rules.d/99-openocd.rules`:
``` console
sudo vi /etc/udev/rules.d/99-openocd.rules
```
内容如下：
``` text
# STM32F3DISCOVERY - ST-LINK/V2.1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", MODE:="0666"
```
#### 对于带有可选USB <-> 基于FT232的串行模块的旧设备

创建 `/etc/udev/rules.d/99-ftdi.rules`:
``` console
sudo vi /etc/udev/rules.d/99-openocd.rules
```
内容如下：
``` text
# FT232 - USB <-> Serial Converter
ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6001", MODE:="0666"
```

### 重新加载udev规则：

``` console
sudo udevadm control --reload-rules
```

如果您的电脑上插了任何板，请拔下它们的插头，然后重新插入。

现在，转到[下一节]。

[下一节]: verify.md
