# 验证安装

让我们验证所有工具是否正确安装。

## 仅限Linux

### 验证权限

用USB电缆将STM32F3DISCOVERY连接到计算机。确保将电缆连接到"USB ST-LINK"端口，即板边缘中央的USB端口。

STM32F3DISCOVERY现在应显示为`/dev/bus/usb`中的USB设备（文件）。让我们来看看它是如何被枚举的：

``` console
lsusb | grep -i stm
```
这将导致：
``` console
$ lsusb | grep -i stm
Bus 003 Device 004: ID 0483:374b STMicroelectronics ST-LINK/V2.1
$ # ^^^        ^^^
```

在我的例子中，STM32F3DISCOVERY连接到总线#3并被枚举为设备#4。这意味着文件
`/dev/bus/usb/003/004`*是*STM32F3DISCOVERY。让我们检查一下它的权限：
``` console
$ ls -la /dev/bus/usb/003/004
crw-rw-rw-+ 1 root root 189, 259 Feb 28 13:32 /dev/bus/usb/003/00
```

权限应为`crw-rw-rw-`。如果不是...请检查[udev规则]并尝试重新加载它们：

[udev规则]: linux.md#udev-rules

``` console
sudo udevadm control --reload-rules
```

#### 对于带有可选USB USB <-> 基于FT232的串行模块的旧设备

拔下STM32F3DISCOVERY并插入串行模块。现在，找出它的关联文件是什么：

``` console
$ lsusb | grep -i ft232
Bus 003 Device 005: ID 0403:6001 Future Technology Devices International, Ltd FT232 Serial (UART) IC
```

在我的例子中，它是`/dev/bus/usb/003/005`。现在，检查它的权限：

``` console
$ ls -l /dev/bus/usb/003/005
crw-rw-rw- 1 root root 189, 21 Sep 13 00:00 /dev/bus/usb/003/005
```

如前所述，权限应为`crw-rw-rw-`。

## 验证 OpenOCD 连接

使用USB电缆将STM32F3DISCOVERY连接到板边缘中央标有"USB ST-LINK"的USB端口。

将USB电缆连接到电路板后，两个*红色*LEDs应立即亮起。

> **重要信息**：STM32F3DISCOVERY板有多个硬件版本。对于较旧的版本，您需要将"interface"参数更改为
> `-f interface/stlink-v2.cfg` (注意: 末尾没有`-1`)。或者，旧版本可以使用`-f board/stm32f3discovery.cfg`
> 而不是`-f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`。

> **注意**：OpenOCD v0.11.0 已弃用`interface/stlink-v2.cfg`而改用`interface/stlink.cfg`
> 支持ST-LINK/V1, ST-LINK/V2, ST-LINK/V2-1 和 ST-LINK/V3。

### *Nix

> **仅供参考:**：`interface`目录通常位于`/usr/share/openocd/scripts/`中，这是openocd期望这些文件的默认位置。
> 如果您在其他地方安装了它们，请使用`-s /path/to/scripts/`选项指定安装目录。

``` console
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

或者

``` console
openocd -f interface/stlink.cfg -f target/stm32f3x.cfg
```


### Windows

在`C:\OpenOCD`的引用下面是OpenOCD安装的目录。

``` console
openocd -s C:\OpenOCD\share\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

> **注意**：cygwin用户报告了-s标志。如果遇到该问题，可以将`C:\OpenOCD\share\scripts\`添加到参数中。

cygwin 用户：
``` console
openocd -f C:\OpenOCD\share\scripts\interface\stlink-v2-1.cfg -f C:\OpenOCD\share\scripts\target\stm32f3x.cfg
```

### 全部

OpenOCD是一种将调试信息从ITM通道转发到文件ITM的服务。
`itm.txt`,因此它将永远运行，**不**会返回到终端提示符。

OpenOCD的初始输出类似于：
``` console
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
none separate
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v27 API v2 SWIM v15 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.915608
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

(如果没有...请查看[一般故障排除]说明。)

[一般故障排除]: ../appendix/1-general-troubleshooting/index.html

此外，最靠近USB端口的一个红色LED应该开始在红光和绿光之间变化。

就是这样！它起作用了。现在可以使用`Ctrl-c`停止OpenOCD或关闭/终止终端。
