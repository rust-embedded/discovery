# The challenge

You are now well armed to face a challenge! Your task will be to implement the
application I showed you at the beginning of this chapter.

Here's the GIF again:

<p align="center">
<img src="https://i.imgur.com/0k1r2Lc.gif">
</p>

Also, this may help:

<p align="center">
<img src="assets/timing-diagram.png">
</p>

This is a timing diagram. It indicates which LED is on at any given instant of
time and for how long each LED should be on. On the X axis we have the time in
milliseconds. The timing diagram shows a single period. This pattern will repeat
itself every 800 ms. The Y axis labels each LED with a cardinal point: North,
East, etc. As part of the challenge you'll have to figure out how each element
in the `LEDS` array maps to these cardinal points (hint: `cargo doc --open`
`;-)`).

Before you attempt this challenge, let me give you one last tip. Our GDB
sessions always involve entering the same commands at the beginning. We can use
a `.gdbinit` file to execute some commands right after GDB is started. This way
you can save yourself the effort of having to enter them manually on each GDB
session.

Place this `.gdbinit` file at the root of the Cargo project, right next to the
`Cargo.toml`:

```
target remote :3333
load
break main
continue
```

With that in place, you should now be able to start a `gdb` session that will
automatically flash the program and jump to the beginning of `main`:

```
$ arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...done.
(..)
Loading section .text, size 0x2014 lma 0x8000000
Start address 0x8000194, load size 8212
Transfer rate: 15 KB/sec, 8212 bytes/write.
Breakpoint 1 at 0x80001e6: file $PWD/src/main.rs, line 12.
Note: automatically using hardware breakpoints for read-only addresses.

Breakpoint 1, led_roulette::main () at $PWD/src/main.rs:12
12      pub fn main() -> ! {
(gdb)
```

But if that doesn't work and, instead, you get this:

```
$ arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...done.
warning: File "$PWD/.gdbinit" auto-loading has been declined by your `auto-load safe-path' set to "$debugdir:$datadir/auto-load".
To enable execution of this file add
        add-auto-load-safe-path $PWD/.gdbinit
line to your configuration file "$HOME/.gdbinit".
To completely disable this security protection add
        set auto-load safe-path /
line to your configuration file "$HOME/.gdbinit".
For more information about this security protection see the
"Auto-loading safe path" section in the GDB manual.  E.g., run from the shell:
        info "(gdb)Auto-loading safe path"
```

You'll have to do a few extra steps. It's definitively worth it though.

## *nix

This command should do the trick.

```
$ echo 'set auto-load safe-path /' > ~/.gdbinit
```

The project local `.gdbinit` should work now.

## Windows

AFAIK, Windows doesn't set a `%HOME%` env variable by default so you'll have to
add that variable to your environment first. I recommend you set it to
`%USERPROFILE%` (e.g. `C:\Users\japaric`).

Then you have to create a `.gdbinit` file in `%HOME%` (e.g.
`C:\Users\japaric\.gdbinit`) with these contents:

```
set auto-load safe-path /
```

The project local `.gdbinit` should work now.
