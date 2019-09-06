# macOS

All the tools can be install using [Homebrew]:

[Homebrew]: http://brew.sh/

``` console
$ brew install minicom openocd
```

The `gcc-arm-embedded` are no longer available through [Homebrew]. The
following is a step by step process for getting the ARM GCC cross
compiler installed on OSX. Also available in Joe Goggins' [Gist] as
referenced in [issue #118].

[Gist]: https://gist.github.com/joegoggins/7763637
[issue #118]: https://github.com/rust-embedded/book/issues/118

Download the latest GNU Embedded Toolchain from the ARM Developer site
using the following URL.

    https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads

The download URL is ridiculously long and you will need to visit the site in
a browser. Once there clicking on the download button will provide a list of
architectures that you can download for and you simply need to select "Mac
OS X 64-bit".

Once downloaded, you will find the the file in your `~/Download` folder. It
will be something like `gcc-arm-none-eabi-8-2019-q3-update-mac.tar.bz2`. To
install the toolchain execute the following commands in a terminal:

```console
mkdir -p /usr/local/gcc_arm
cd /usr/local/gcc_arm
tar xjf ~/Downloads/<DOWNLOADED_FILE>
```

_Note_: Depending on the permissions of `/usr/local`, you may need to
preface the `mkdir` and `tar` commands with `sudo`.

You will then need to add `/usr/local/gcc_arm/gcc-arm-none-eabi-8-2019-q3-update/bin`
to your `$PATH`. If you have a newer distribution of the toolchain, your
directory under `/usr/local/gcc_arm` will be different and you will need
to adjust the above path.

The `$PATH` variable can be changed temporarily for the current terminal
by executing the following:

```console
export PATH=/usr/local/gcc_arm/gcc-arm-none-eabi-8-2019-q3-update/bin:$PATH
```

As soon as the terminal is closed, the directory will not be in your `$PATH`
any longer and the above command will need to be executed again in a new
terminal window.

To make the change take effect for all terminal windows, you will need to
update your `~/.bash_profile` or `~/.zprofile` depending upon which shell
you are running. You can execute `echo $SHELL` to determine which one you
have. Add the above `export` command to your appropriate file and save it.
For current terminal sessions you will need to execute `source ~/.zprofile`
or `source ~/.bash_profile`. Once you logout and back in again, the change
will be made in all terminal sessions. 

That's all! Go to the [next section].

[next section]: verify.md
