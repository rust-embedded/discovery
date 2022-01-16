# Linux

<!-- Here are the installation commands for a few Linux distributions. -->

いくつかのLinuxディストリビューションのインストールコマンドを示します。

<!-- ## REQUIRED packages -->

## 必須のパッケージ

<!-- - Ubuntu 18.04 or newer / Debian stretch or newer -->

- Ubuntu 18.04以上 / Debian stretch以降
<!-- 
> **NOTE** `gdb-multiarch` is the GDB command you'll use to debug your ARM
> Cortex-M programs
 -->

**注記** `gdb-multiarch`は、ARM Cortex-Mプログラムをデバッグするために使用するGDBのコマンドです。

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

<!-- - Ubuntu 14.04 and 16.04 -->

- Ubuntu 14.04と16.04

<!-- 
> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs
 -->

**注記** `arm-none-eabi-gdb`は、ARM Cortex-Mプログラムをデバッグするために使用するGDBのコマンドです。

<!-- Ubuntu 14.04 -->
<!-- GDB 7.6 -->
<!-- OpenOCD 0.7.0 -->

``` console
sudo apt-get install \
  gdb-arm-none-eabi \
  minicom \
  openocd
```

<!-- - Fedora 23 or newer -->

- Fedora 23以上

<!-- 
> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your ARM
> Cortex-M programs
 -->

**注記** `arm-none-eabi-gdb`は、ARM Cortex-Mプログラムをデバッグするために使用するGDBのコマンドです。

``` console
sudo dnf install \
  minicom \
  openocd \
  gdb
```

### Arch Linux

<!-- 
> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug ARM
> Cortex-M programs
 -->

**注記** `arm-none-eabi-gdb`は、ARM Cortex-Mプログラムをデバッグするために使用するGDBのコマンドです。

``` console
sudo pacman -S \
  arm-none-eabi-gdb \
  minicom \
  openocd
```

<!-- 
`openocd` is not available in the official Arch repositories, but can be installed from the [AUR](https://aur.archlinux.org/packages/openocd/) or can be compiled from source as follows:
 -->

`openocd`は、公式のArchレポジトリから入手できません。しかし、[AUR](https://aur.archlinux.org/packages/openocd/)からインストールするか、
次のようにソースからコンパイルすることができます。

``` console
git clone git://git.code.sf.net/p/openocd/code openocd-code
cd openocd-code
./bootstrap
./configure
make && make install
```

<!-- - `arm-none-eabi-gdb` for other distros -->

- 他ディストリビューションにおける`arm-none-eabi-gdb`

<!-- 
For distros that don't have packages for [ARM's pre-built
toolchain](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads),
download the "Linux 64-bit" file and put its `bin` directory on your path.
Here's one way to do it:
 -->

パッケージが用意されていないディストリビューションについては、
[ARM's pre-built toolchain](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)から「Linux 64-bit」ファイルをダウンロードして、
`bin`ディレクトリにパスを通して下さい。
下記は、やり方の1例です。

``` console
mkdir -p ~/local && cd ~/local
```
``` console
tar xjf /path/to/downloaded/file/gcc-arm-none-eabi-10-2020-q4-major-x86_64-linux.tar.bz2
```

<!-- 
Then, use your editor of choice to append to your `PATH` in the appropriate
shell init file (e.g. `~/.zshrc` or `~/.bashrc`):
 -->

次に、エディタを使って、適切なシェル初期化ファイル（例えば、`~/.zshrc`や`~/.bashrc`）の`PATH`を追加して下さい。

```
PATH=$PATH:$HOME/local/gcc-arm-none-eabi-10-2020-q4-major-x86_64-linux/bin
```

<!-- ## Optional packages -->

## オプションのパッケージ

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

<!-- ## udev rules -->

## udevルール

<!-- 
These rules let you use USB devices like the F3 and the Serial module without root privilege, i.e.
`sudo`.
 -->

このルールにより、ルート権限（つまり`sudo`）なしで、F3とシリアルモジュールのようなUSBデバイスを使えるようにします。

<!-- Create these two files in `/etc/udev/rules.d` with the contents shown below. -->

下記の内容で、`/etc/udev/rules.d`ディレクトリに2つのファイルを作成します。

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

<!-- Then reload the udev rules with: -->

その後、udevルールをリロードします。

``` console
sudo udevadm control --reload-rules
```

<!-- If you had any board plugged to your laptop, unplug them and then plug them in again. -->

既にボードをノートPCに接続している場合、一度抜いてから、もう一度接続します。

<!-- Now, go to the [next section]. -->

それでは、[次のセクション]に進んで下さい。

<!-- [next section]: verify.md -->

[次のセクション]: verify.md
