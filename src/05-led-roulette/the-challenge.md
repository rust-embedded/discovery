# The challenge

You are now well armed to face a challenge! Your task will be to implement the application I showed
you at the beginning of this chapter.

Here's the GIF again:

<p align="center">
<img src="https://i.imgur.com/0k1r2Lc.gif">
</p>

Also, this may help:

<p align="center">
<img class="white_bg" src="../assets/timing-diagram.png">
</p>

This is a timing diagram. It indicates which LED is on at any given instant of time and for how long
each LED should be on. On the X axis we have the time in milliseconds. The timing diagram shows a
single period. This pattern will repeat itself every 800 ms. The Y axis labels each LED with a
cardinal point: North, East, etc. As part of the challenge you'll have to figure out how each
element in the `Leds` array maps to these cardinal points (hint: `cargo doc --open` `;-)`).

Before you attempt this challenge, let me give you one additonal tip. Our GDB sessions always involve
entering the same commands at the beginning. We can use a `.gdb` file to execute some commands
right after GDB is started. This way you can save yourself the effort of having to enter them
manually on each GDB session.

As it turns out we've already created `../openocd.gdb` and you can see it's doing
pretty much what we did in the previous section plus a few other commands. Look at
the comments for additional information:

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

Now we need to modify the `../.cargo/config.toml` file to execute `../openocd.gdb`
``` console
nano ../.cargo/config.toml
```

Edit your `runner` command ` -x ../openocd.gdb`.
Assuming you're using `arm-none-eabi-gdb` the diff is:
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

And the full contents of `../.cargo/config.toml`, again
assuming `arm-none-eabi-gdb`, is:
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

With that in place, you can now use a simple `cargo run` command which will build
the ARM version of the code and run the `gdb` session. The `gdb` session will
automatically flash the program and jump to the beginning of `main` as it `step`'s
through the entry trampoline:

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

If you haven't already ready, it's probably a good idea to fork
the [embedded discovery book](https://github.com/rust-embedded/discovery) so you
can save your changes in your own branch of your fork. We suggest creating
your own branch and leaving the `master` branch alone so the `master` branch
of your fork can stay in sync with the upstream repo. Also, it allows you to
more easily create PR's and improve this book, **thank you in advance**!
