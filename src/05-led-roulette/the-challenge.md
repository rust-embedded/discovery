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

Before you attempt this challenge, let me give you one last tip. Our GDB sessions always involve
entering the same commands at the beginning. We can use a `.gdb` file to execute some commands
right after GDB is started. This way you can save yourself the effort of having to enter them
manually on each GDB session.

Place this `openocd.gdb` file in the root of the Cargo project, right next to the `Cargo.toml`:

``` console
$ cat openocd.gdb
```

``` text
target remote :3333
load
break main
continue
```

Then modify the second line of the `.cargo/config` file:

``` console
$ cat .cargo/config
```

``` toml
[target.thumbv7em-none-eabihf]
runner = "arm-none-eabi-gdb -q -x openocd.gdb" # <-
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

With that in place, you should now be able to start a `gdb` session that will automatically flash
the program and jump to the beginning of `main`:

``` console
$ cargo run --target thumbv7em-none-eabihf
     Running `arm-none-eabi-gdb -q -x openocd.gdb target/thumbv7em-none-eabihf/debug/led-roulette`
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...done.
(..)
Loading section .vector_table, size 0x188 lma 0x8000000
Loading section .text, size 0x3b20 lma 0x8000188
Loading section .rodata, size 0xb0c lma 0x8003cc0
Start address 0x8003b1c, load size 18356
Transfer rate: 20 KB/sec, 6118 bytes/write.
Breakpoint 1 at 0x800018c: file src/05-led-roulette/src/main.rs, line 9.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, main () at src/05-led-roulette/src/main.rs:9
9           let (mut delay, mut leds): (Delay, Leds) = aux5::init();
(gdb)
```
