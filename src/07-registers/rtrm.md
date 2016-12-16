# RTRM: Reading The Reference Manual

I mentioned that the microcontroller has several pins. For convenience, these
pins are grouped in "ports" of 16 pins. Each port is named with a letter: Port
A, Port B, etc. and the pins within each port are named with numbers from 0
to 15.

The first thing we have to find out is which pin is connected to which LED. This
information is in the STM32F3DISCOVERY [User Manual] (You downloaded a copy,
right?). In this particular section:

[User Manual]: http://www.st.com/resource/en/user_manual/dm00063382.pdf

> Section 6.4 LEDs - Page 18

The manual says:

- `LD3`, the North LED, is connected to the pin `PE9`. `PE9` is the short form
  of: Pin 9 on Port E.
- `LD7`, the East LED, is connected to the pin `PE11`.

Up to this point, we know that we want to change the state of the pins PE9 and
PE11 to turn the North/East LEDs on/off. These pins are part of Port E so we'll
have to deal with the `GPIOE` peripheral.

Each peripheral has a register "block" associated to it. A register block is a
collection of registers allocated in contiguous memory. The address at which
the register block starts is known as its base address. We need to figure out
what's the base address of the `GPIOE` peripheral. That information is in the
following section of the microcontroller [Reference Manual]:

[Reference Manual]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf

> Section 3.2.2 Memory map and register boundary addresses - Page 51

The table says that base address of the `GPIOE` register block is `0x4800_1000`.

Each peripheral also has its own section in the documentation. Each of these
sections ends with a table of the registers that the peripheral's register block
contains. For the `GPIO` family of peripheral, that table is in:

> Section 11.4.12 GPIO register map - Page 243

We are interested in the register that's at an offset of `0x18` from the base
address of the `GPIOE` peripheral. According to the table, that would be the
register `BSRR`.

Now we need to jump to the documentation of that particular register. It's a few
pages above in:

> Section 11.4.7 GPIO port bit set/reset register (GPIOx_BSRR) - Page 240

Finally!

This is the register we were writing to. The documentation says some interesting
things. First, this register is write only ... so let's try reading its value
`:-)`.

We'll use GDB's "examine" command: `x`.

```
(gdb) next
14              *(GPIOE_BSRR as *mut u32) = 1 << 9;

(gdb) x 0x48001018
0x48001018:     0x00000000

(gdb) next
17              *(GPIOE_BSRR as *mut u32) = 1 << 11;

(gdb) x 0x48001018
0x48001018:     0x00000000
```

Reading the register returns `0`. That matches what the documentation says.

The other thing that the documentation says is that the bits 0 to 15 can be used
to "set" the corresponding pin. That is bit 0 "sets" the pin 0. Here, "set"
means outputting a "high" value on the pin.

The documentation also says that bits 16 to 31 can be used to "reset" the
corresponding pin. In this case, the bit 16 resets the pin number 0. As you may
guess, "reset" means outputting a "low" value on the pin.

Correlating that information with our program, all seems to be in agreement:

- Writing `1 << 9` (`BS9 = 1`)  to `BSRR`  sets `PE9` *high*. That turns the
  North LED *on*.

- Writing `1 << 11` (`BS11 = 1`) to `BSRR` sets `PE11` *high*. That turns the
  East LED *on*.

- Writing `1 << 25` (`BR9 = 1`) to `BSRR` sets `PE9` *low*. That turns the
  North LED *off*.

- Finally, writing `1 << 27` (`BR11 = 1`) to `BSRR` sets `PE11` *low*. That
  turns the East LED *off*.
