# Windows

## `arm-none-eabi-gdb`

ARM provides `.exe` installers for Windows. Grab one from [here][gcc], and follow the instructions.
Just before the installation process finishes tick/select the "Add path to environment variable"
option. Then verify that the tools are in your `%PATH%`:

``` console
$ arm-none-eabi-gcc -v
(..)
gcc version 5.4.1 20160919 (release) (..)
```

[gcc]: https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads

## PuTTY

Download the latest `putty.exe` from [this site] and place it somewhere in your `%PATH%`.

[this site]: http://www.chiark.greenend.org.uk/~sgtatham/putty/download.html

Now, go to the [next section].

[next section]: verify.md
