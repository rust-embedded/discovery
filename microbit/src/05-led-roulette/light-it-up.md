# Light it up
## embedded-hal

In this chapter we are going to make one of the many LEDs on the back of the micro:bit light up since this is
basically the "Hello World" of embedded programming. In order to get this task done we will use one of the traits
provided by `embedded-hal`, specifically the [`OutputPin`] trait which allows us to turn a pin on or off.

[`OutputPin`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/digital/v2/trait.OutputPin.html

## The micro:bit LEDs

On the back of the micro:bit you can see a 5x5 square of LEDs, usually called an LED matrix. This matrix alignment is
used so that instead of having to use 25 separate pins to drive every single one of the LEDs, we can just use 10 (5+5) pins in
order to control which column and which row of our matrix lights up.

> **NOTE** that the micro:bit v1 team implemented this a little differently. Their [schematic page] says
> that it is actually implemented as a 3x9 matrix but a few columns simply remain unused.

Usually in order to determine which specific pins we have to control in
order to light a specific LED up we would now have to read the
[micro:bit v2 schematic] or the [micro:bit v1 schematic] respectively.
Luckily for us though we can use the aforementioned micro:bit BSP
which abstracts all of this nicely away from us.

[schematic page]: https://tech.microbit.org/hardware/schematic/
[micro:bit v2 schematic]: https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.00/MicroBit_V2.0.0_S_schematic.PDF
[micro:bit v1 schematic]: https://github.com/bbcmicrobit/hardware/blob/master/V1.5/SCH_BBC-Microbit_V1.5.PDF

## Actually lighting it up!

The code required to light up an LED in the matrix is actually quite simple but it requires a bit of setup. First take
a look at it and then we can go through it step by step:

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::board::Board;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    loop {}
}
```

The first few lines until the main function just do some basic imports and setup we already looked at before.
However, the main function looks pretty different to what we have seen up to now.

The first line is related to how most HALs written in Rust work internally.
As discussed before they are built on top of PAC crates which own (in the Rust sense)
all the peripherals of a chip. `let mut board = Board::take().unwrap();` basically takes all
these peripherals from the PAC and binds them to a variable. In this specific case we are
not only working with a HAL but with an entire BSP, so this also takes ownership
of the Rust representation of the other chips on the board.

> **NOTE**: If you are wondering why we have to call `unwrap()` here, in theory it is possible for `take()` to be called
> more than once. This would lead to the peripherals being represented by two separate variables and thus lots of
> possible confusing behaviour because two variables modify the same resource. In order to avoid this, PACs are
> implemented in a way that it would panic if you tried to take the peripherals twice.

Now we can light the LED connected to `row1`, `col1` up by setting the `row1` pin to high (i.e. switching it on).
The reason we can leave `col1` set to low is because of how the LED matrix circuit works. Furthermore, `embedded-hal` is
designed in a way that every operation on hardware can possibly return an error, even just toggling a pin on or off. Since
that is highly unlikely in our case, we can just `unwrap()` the result.

## Testing it

Testing our little program is quite simple. First put it into `src/main.rs`. Afterwards we simply have to run the
`cargo embed` command from the last section again, let it flash and just like before. Then open our GDB and connect
to the GDB stub:

```
$ # Your GDB debug command from the last section
(gdb) target remote :1337
Remote debugging using :1337
cortex_m_rt::Reset () at /home/nix/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.12/src/lib.rs:489
489     pub unsafe extern "C" fn Reset() -> ! {
(gdb)
```

If we now let the program run via the GDB `continue` command, one of the LEDs on the back of the micro:bit should light
up.
