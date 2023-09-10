# Windows

## `arm-none-eabi-gdb`

ARM`.exe`为Windows提供安装程序。从[从][gcc]下载，然后按照说明进行操作。就在安装过程完成之前
勾选/选择"Add path to environment variable"选项。然后验证这些工具是否在您的`%PATH%`：

``` console
$ arm-none-eabi-gcc -v
(..)
gcc version 5.4.1 20160919 (release) (..)
```

[gcc]: https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads

## PuTTY

`putty.exe`从[该站点]下载最新版本并将其放在您的`%PATH%`。

现在，转到[下一节]。

[该站点]: http://www.chiark.greenend.org.uk/~sgtatham/putty/download.html

[下一节]: verify.md
