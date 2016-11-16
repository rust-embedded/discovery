# Windows

## `arm-none-eabi-*`

The GNU ARM Embedded Toolchain project provides `.exe` installers for Windows.
Grab [this one], and follow the instructions. Just before the installation
process finishes tick/select the "Add path to environment variable" option. Then
verify that the tools are in your `%PATH%`:
```
$ arm-none-eabi-gcc -v
(..)
gcc version 5.4.1 20160919 (release) (..)
```

[GNU ARM Embedded Toolchain]: https://launchpad.net/gcc-arm-embedded/+download
[this one]: https://launchpad.net/gcc-arm-embedded/5.0/5-2016-q3-update/+download/gcc-arm-none-eabi-5_4-2016q3-20160926-win32.exe

## OpenOCD

There's no official binary release of OpenOCD for Windows but there are
unofficial releases available [here]. Grab the 0.9.0 zipfile and extract it
somewhere in your drive (I recommend `C:\OpenOCD` but with the drive letter that
makes sense to you) then update your `%PATH%` environment variable to include
the following path: `C:\OpenOCD\bin` (or the path that you used before).

[here]: http://gnutoolchains.com/arm-eabi/openocd/

Verify that OpenOCD is in yout `%PATH%` with:

```
$ openocd -v
Open On-Chip Debugger 0.9.0 2015-08-15-12:41 (..)
```

## PuTTY

Download the latest `putty.exe` from [this site] and place it somewhere in your
`%PATH%`.

[this site]: http://www.chiark.greenend.org.uk/~sgtatham/putty/download.html

## ST-LINK USB driver

You'll also need to install [this USB driver] or OpenOCD won't work. Follow the
installer instructions and make sure you install the right (32-bit or 64-bit)
version of the driver.

[this USB driver]: http://www.st.com/en/embedded-software/stsw-link009.html

That's all! Go to the [next section].

[next section]: 03-setup/verify.html
