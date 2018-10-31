# Linux

Here are the installation commands for a few Linux distributions.

## REQUIRED packages

- Ubuntu 16.04 or newer / Debian Jessie or newer

``` console
$ sudo apt-get install \
  gdb-arm-none-eabi \
  minicom \
  openocd
```

- Fedora 23 or newer

``` console
$ sudo dnf install \
  arm-none-eabi-gdb \
  minicom \
  openocd
```

- Arch Linux

``` console
$ sudo pacman -S \
  arm-none-eabi-gdb \
  minicom \
  openocd
```

- `arm-none-eabi-gdb` for other distros

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

## Optional packages

- Ubuntu / Debian

``` console
$ sudo apt-get install \
  bluez \
  rfkill
```

- Fedora

``` console
$ sudo dnf install \
  bluez \
  rfkill
```

- Arch Linux

``` console
$ sudo pacman -S \
  bluez \
  bluez-utils \
  rfkill
```

## udev rules

These rules let you use USB devices like the F3 and the Serial module without root privilege, i.e.
`sudo`.

Create these two files in `/etc/udev/rules.d` with the contents shown below.

``` console
$ cat /etc/udev/rules.d/99-ftdi.rules
```

``` text
# FT232 - USB <-> Serial Converter
ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6001", MODE:="0666"
```

``` console
$ cat /etc/udev/rules.d/99-openocd.rules
```

``` text
# STM32F3DISCOVERY rev A/B - ST-LINK/V2
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", MODE:="0666"

# STM32F3DISCOVERY rev C+ - ST-LINK/V2-1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", MODE:="0666"
```

Then reload the udev rules with:

``` console
$ sudo udevadm control --reload-rules
```

If you had any board plugged to your laptop, unplug them and then plug them in again.

Now, go to the [next section].

[next section]: verify.md
