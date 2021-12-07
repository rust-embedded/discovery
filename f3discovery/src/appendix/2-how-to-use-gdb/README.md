# How to use GDB

Below are some useful GDB commands that can help us debug our programs. This assumes you have [flashed a program](../../05-led-roulette/flash-it.md) onto your microcontroller and attached to an OpenOCD session.

## General Debugging

> **NOTE:** Many of the commands you see below can be executed using a short form. For example, `continue` can simply be used as `c`, or `break $location` can be used as `b $location`. Once you have experience with the commands below, try to see how short you can get the commands to go before GDB doesn't recognize them!


### Dealing with Breakpoints

* `break $location`: Set a breakpoint at a place in your code. The value of `$location` can include:
    * `break *main` - Break on the exact address of the function `main`
    * `break *0x080012f2` - Break on the exact memory location `0x080012f2`
    * `break 123` - Break on line 123 of the currently displayed file
    * `break main.rs:123` - Break on line 123 of the file `main.rs`
* `info break`: Display current breakpoints
* `delete`: Delete all breakpoints
    * `delete $n`: Delete breakpoint `$n` (`n` being a number. For example: `delete $2`)
* `clear`: Delete breakpoint at next instruction
    * `clear main.rs:$function`: Delete breakpoint at entry of `$function` in `main.rs`
    * `clear main.rs:123`: Delete breakpoint on line 123 of `main.rs`
* `enable`: Enable all set breakpoints
  * `enable $n`: Enable breakpoint `$n`
* `disable`: Disable all set breakpoints
  * `disable $n`: Disable breakpoint `$n`

### Controlling Execution

* `continue`: Begin or continue execution of your program
* `next`: Execute the next line of your program
    * `next $n`: Repeat `next` `$n` number times
* `nexti`: Same as `next` but with machine instructions instead
* `step`: Execute the next line, if the next line includes a call to another function, step into that code
    * `step $n`: Repeat `step` `$n` number times
* `stepi`: Same as `step` but with machine instructions instead
* `jump $location`: Resume execution at specified location:
    * `jump 123`: Resume execution at line 123
    * `jump 0x080012f2`: Resume execution at address 0x080012f2

### Printing Information

* `print /$f $data` - Print the value contained by the variable `$data`. Optionally format the output with `$f`, which can include:
    ```txt
    x: hexadecimal 
    d: signed decimal
    u: unsigned decimal
    o: octal
    t: binary
    a: address
    c: character
    f: floating point
    ```
    * `print /t 0xA`: Prints the hexadecimal value `0xA` as binary (0b1010)
* `x /$n$u$f $address`: Examine memory at `$address`. Optionally, `$n` define the number of units to display, `$u` unit size (bytes, halfwords, words, etc), `$f` any `print` format defined above
    * `x /5i 0x080012c4`: Print 5 machine instructions staring at address `0x080012c4`
    * `x/4xb $pc`: Print 4 bytes of memory starting where `$pc` currently is pointing
* `disassemble $location`
    * `disassemble /r main`: Disassemble the function `main`, using `/r` to show the bytes that make up each instruction


### Looking at the Symbol Table

* `info functions $regex`: Print the names and data types of functions matched by `$regex`, omit `$regex` to print all functions
    * `info functions main`: Print names and types of defined functions that contain the word `main`
* `info address $symbol`: Print where `$symbol` is stored in memory
    * `info address GPIOC`: Print the memory address of the variable `GPIOC`
* `info variables $regex`: Print names and types of global variables matched by `$regex`, omit `$regex` to print all global variables
* `ptype $data`: Print more detailed information about `$data`
    * `ptype cp`: Print detailed type information about the variable `cp` 

### Poking around the Program Stack

* `backtrace $n`: Print trace of `$n` frames, or omit `$n` to print all frames
  * `backtrace 2`: Print trace of first 2 frames
* `frame $n`: Select frame with number or address `$n`, omit `$n` to display current frame
* `up $n`: Select frame `$n` frames up
* `down $n`: Select frame `$n` frames down
* `info frame $address`: Describe frame at `$address`, omit `$address` for currently selected frame
* `info args`: Print arguments of selected frame
* `info registers $r`: Print the value of register `$r` in selected frame, omit `$r` for all registers
    * `info registers $sp`: Print the value of the stack pointer register `$sp` in the current frame

### Controlling OpenOCD Remotely

* `monitor reset run`: Reset the CPU, starting execution over again
    * `monitor reset`: Same as above
* `monitor reset init`: Reset the CPU, halting execution at the start
* `monitor targets`: Display information and state of current target
