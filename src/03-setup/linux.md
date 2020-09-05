# Linux

Here are the installation commands for a few Linux distributions.

## Ubuntu 18.04 or newer / Debian stretch or newer

> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs

<!-- Debian stretch -->
<!-- GDB 7.12 -->
<!-- OpenOCD 0.9.0 -->

<!-- Ubuntu 18.04 -->
<!-- GDB 8.1 -->
<!-- OpenOCD 0.10.0 -->

``` console
$ sudo apt-get install \
  arm-none-eabi-gdb \
  minicom
```

## Ubuntu 14.04 and 16.04

> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs

<!-- Ubuntu 14.04 -->
<!-- GDB 7.6 -->
<!-- OpenOCD 0.7.0 -->

``` console
$ sudo apt-get install \
  gdb-arm-none-eabi \
  minicom
```

## Fedora 23 or newer

``` console
$ sudo dnf install \
  gdb \
  minicom
```

## Arch Linux

> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs

``` console
$ sudo pacman -S \
  arm-none-eabi-gdb \
  minicom
```

## Other distros

> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs

For distros that don't have packages for [ARM's pre-built
toolchain](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads),
download the "Linux 64-bit" file and put its `bin` directory on your path.
Here's one way to do it:

``` console
$ mkdir -p ~/local && cd ~/local
$ tar xjf /path/to/downloaded/file/gcc-arm-none-eabi-7-2017-q4-major-linux.tar.bz2.tbz
```

Then, use your editor of choice to append to your `PATH` in the appropriate
shell init file (e.g. `~/.zshrc` or `~/.bashrc`):

```
PATH=$PATH:$HOME/local/gcc-arm-none-eabi-7-2017-q4-major/bin
```

## udev rules

These rules let you use USB devices like the micro:bit without root privilege, i.e. `sudo`.

Create this file in `/etc/udev/rules.d` with the content shown below.

``` console
$ cat /etc/udev/rules.d/99-microbit.rules
```

``` text
# CMSIS-DAP for microbit
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
```

Then reload the udev rules with:

``` console
$ sudo udevadm control --reload-rules
```

If you had any board plugged to your computer, unplug them and then plug them in again.

Now, go to the [next section].

[next section]: verify.md
