# Windows

## `arm-none-eabi-gdb`

ARM为Windows提供`.exe`安装程序。从[这里][gcc]获取，并按照说明操作。就在安装过程完成之前，勾选/选择
"向环境变量添加路径"选项。然后验证工具是否在`%PATH%`中：

验证是否安装了gcc：
``` console
arm-none-eabi-gcc -v
```
结果应该是：
```
(..)
$ arm-none-eabi-gcc -v
gcc version 5.4.1 20160919 (release) (..)
```

[gcc]: https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads

## OpenOCD

OpenOCD for Windows没有官方二进制版本，但[这里][openocd]有非官方版本。获取0.10.x zip文件并将其解压缩到
驱动器中的某个位置 (我建议使用`C:\OpenOCD`但使用对您有意义的驱动器号) 然后更新`%PATH%`环境变量，使其包含
以下路径：`C:\OpenOCD\bin` (或以前使用的路径)。

[openocd]: https://github.com/xpack-dev-tools/openocd-xpack/releases

验证OpenOCD是否已安装并在`%PATH%`中：
``` console
openocd -v
```
结果应该是：
``` console
$ openocd -v
Open On-Chip Debugger 0.10.0
(..)
```

## PuTTY

从[此网站]下载最新的`putty.exe`并将其放置在`%PATH%`中的某个位置。

[此网站]: http://www.chiark.greenend.org.uk/~sgtatham/putty/download.html

## ST-LINK USB 驱动程序

您还需要安装此[USB驱动程序]，否则OpenOCD将无法工作。按照安装程序说明进行操作，确保安装正确的（32位或64位）驱动程序版本。

[USB驱动程序]: http://www.st.com/en/embedded-software/stsw-link009.html

这就是全部！转到[下一节]。

[下一节]: verify.md
