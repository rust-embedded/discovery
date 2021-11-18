# Linux

Here are the installation commands for a few Linux distributions.

## REQUIRED packages

### Ubuntu 18.04 or newer / Debian stretch or newer

> **NOTE** `gdb-multiarch` is the GDB command you'll use to debug your ARM
> Cortex-M programs

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

### Ubuntu 14.04 and 16.04

> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs

<!-- Ubuntu 14.04 -->
<!-- GDB 7.6 -->
<!-- OpenOCD 0.7.0 -->

``` console
sudo apt-get install \
  gdb-arm-none-eabi \
  minicom \
  openocd
```

### Fedora 23 or newer

``` console
sudo dnf install \
  minicom \
  openocd \
  gdb
```

### Arch Linux

> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs

``` console
sudo pacman -S \
  arm-none-eabi-gdb \
  minicom \
  openocd
```

### Other distros

> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs

For distros that don't have packages for [ARM's pre-built
toolchain](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads),
download the "Linux 64-bit" file and put its `bin` directory on your path.
Here's one way to do it:

``` console
mkdir -p ~/local && cd ~/local
```
``` console
tar xjf /path/to/downloaded/file/gcc-arm-none-eabi-10-2020-q4-major-x86_64-linux.tar.bz2
```

Then, use your editor of choice to append to your `PATH` in the appropriate
shell init file (e.g. `~/.zshrc` or `~/.bashrc`):

```
PATH=$PATH:$HOME/local/gcc-arm-none-eabi-10-2020-q4-major-x86_64-linux/bin
```

## Optional packages

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

## udev rules

These rules let you use USB devices like the F3 and the Serial module without root privilege, i.e.
`sudo`.

Create `99-openocd.rules` in `/etc/udev/rules.d` using the `idVendor` and `idProduct`
from the `lsusb` output.

For example, connect the STM32F3DISCOVERY to your computer using a USB cable.
Be sure to connect the cable to the "USB ST-LINK" port, the USB port in the
center of the edge of the board.

Execute `lsusb`:
``` console
lsusb | grep ST-LINK
```
It should result in something like:
```
$ lsusb | grep ST-LINK
Bus 003 Device 003: ID 0483:374b STMicroelectronics ST-LINK/V2.1
```
So the `idVendor` is `0483` and `idProduct` is `374b`.

### Create `/etc/udev/rules.d/99-openocd.rules`:
``` console
sudo vi /etc/udev/rules.d/99-openocd.rules
```
With the contents:
``` text
# STM32F3DISCOVERY - ST-LINK/V2.1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", MODE:="0666"
```
#### For older devices with OPTIONAL USB <-> FT232 based Serial Module

Create `/etc/udev/rules.d/99-ftdi.rules`:
``` console
sudo vi /etc/udev/rules.d/99-openocd.rules
```
With the contents:
``` text
# FT232 - USB <-> Serial Converter
ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6001", MODE:="0666"
```

### Reload the udev rules with:

``` console
sudo udevadm control --reload-rules
```

If you had any board plugged to your computer, unplug them and then plug them in again.

Now, go to the [next section].

[next section]: verify.md
