# 挑战

你现在已经准备好面对挑战了！您的任务是实现我在本章开头向您展示的应用程序。

这里是GIF图：

<p>
<img src="https://i.imgur.com/0k1r2Lc.gif">
</p>

此外，这可能有助于：

<p>
<img class="white_bg" src="../assets/timing-diagram.png">
</p>

这是一个时序图。它指示哪个LED在任何给定的时间点点亮，以及每个LED应该点亮多长时间。在X轴上，我们以毫秒为单位显示时间。
时序图显示了单个周期。这种模式将每800毫秒重复一次。Y轴用一个基点标记每个LED：北、东等。
作为挑战的一部分，您必须弄清楚`Leds`阵列中的每个元素如何映射到这些基点 (提示：`cargo doc --open` `;-)`)。

在你尝试这个挑战之前，让我给你一个额外的提示。我们的GDB会话总是在开始时输入相同的命令。
我们可以在gdb启动后立即使用`.gdb`文件执行一些命令。通过这种方式，您可以省去在每个GDB会话中手动输入它们的工作量。

事实证明，我们已经创建了`../openocd.gdb`，您可以看到它的功能与我们在上一节中所做的差不多，外加一些其他命令。查看评论以了解更多信息：

``` console
$ cat ../openocd.gdb
# Connect to gdb remote server
target remote :3333

# Load will flash the code
load

# Eanble demangling asm names on disassembly
set print asm-demangle on

# Enable pretty printing
set print pretty on

# Disable style sources as the default colors can be hard to read
set style sources off

# Initialize monitoring so iprintln! macro output
# is sent from the itm port to itm.txt
monitor tpiu config internal itm.txt uart off 8000000

# Turn on the itm port
monitor itm port 0 on

# Set a breakpoint at main, aka entry
break main

# Set a breakpiont at DefaultHandler
break DefaultHandler

# Set a breakpiont at HardFault
break HardFault

# Continue running and until we hit the main breakpoint
continue

# Step from the trampoline code in entry into main
step

```

现在，我们需要修改`../.cargo/config.toml`要执行的文件`../openocd.gdb`
``` console
nano ../.cargo/config.toml
```

编辑`runner`命令` -x ../openocd.gdb`。假设您使用的是`arm-none-eabi-gdb`，不同的是：
``` diff
~/embedded-discovery/src/05-led-roulette
$ git diff ../.cargo/config.toml
diff --git a/src/.cargo/config.toml b/src/.cargo/config.toml
index ddff17f..02ac952 100644
--- a/src/.cargo/config.toml
+++ b/src/.cargo/config.toml
@@ -1,5 +1,5 @@
 [target.thumbv7em-none-eabihf]
-runner = "arm-none-eabi-gdb -q"
+runner = "arm-none-eabi-gdb -q -x ../openocd.gdb"
 # runner = "gdb-multiarch -q"
 # runner = "gdb -q"
 rustflags = [
```

`../.cargo/config.toml`的全部内容。再次假设`arm-none-eabi-gdb`，是：
``` toml
[target.thumbv7em-none-eabihf]
runner = "arm-none-eabi-gdb -q -x ../openocd.gdb"
# runner = "gdb-multiarch -q"
# runner = "gdb -q"
rustflags = [
  "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv7em-none-eabihf"

```

有了这些，您现在可以使用一个简单的`cargo run` 命令，该命令将构建ARM版本的代码并运行`gdb`会话。
`gdb`会话将自动刷新程序，并在进入`step`时跳转到`main`的开头：

``` console
cargo run
```

``` console
~/embedded-discovery/src/05-led-roulette (Update-05-led-roulette-WIP)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `arm-none-eabi-gdb -q -x openocd.gdb ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette`
Reading symbols from ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette...
led_roulette::__cortex_m_rt_main_trampoline () at ~/embedded-discovery/src/05-led-roulette/src/main.rs:7
7       #[entry]
Loading section .vector_table, size 0x194 lma 0x8000000
Loading section .text, size 0x52c0 lma 0x8000194
Loading section .rodata, size 0xb50 lma 0x8005454
Start address 0x08000194, load size 24484
Transfer rate: 21 KB/sec, 6121 bytes/write.
Breakpoint 1 at 0x8000202: file ~/embedded-discovery/src/05-led-roulette/src/main.rs, line 7.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, led_roulette::__cortex_m_rt_main_trampoline ()
    at ~/embedded-discovery/src/05-led-roulette/src/main.rs:7
7       #[entry]
led_roulette::__cortex_m_rt_main () at ~/embedded-discovery/src/05-led-roulette/src/main.rs:9
9           let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
```

## Fork the discovery book

如果您还没有准备好，最好将[embedded discovery book](https://github.com/rust-embedded/discovery)进行fork，这样您就可以将更改保存在自己的fork中。
我们建议创建自己的分支，并单独保留`master`分支，这样您的fork的`master`分支就可以与上游 保持同步。
此外，它可以让您更轻松地创建PR并改进本书，**提前感谢**！
