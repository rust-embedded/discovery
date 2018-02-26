# Power

Turns out that, to save power, most peripherals start in a powered off state -- that's their state
right after the microcontroller boots.

The Reset and Clock Control (`RCC`) peripheral can be used to power on or off every other
peripheral.

You can find the list of registers in the `RCC` register block in:

> Section 9.4.14 - RCC register map - Page 166 - Reference Manual

The registers that control the power status of other peripherals are:

- `AHBENR`
- `APB1ENR`
- `APB2ENR`

Each bit in these registers controls the power status of a single peripheral, including `GPIOE`.

Your task in this section is to power on the `GPIOE` peripheral. You'll have to:

- Figure out which of the three registers I mentioned before has the bit that controls the power
  status.
- Figure out what value that bit must be set to,`0` or `1`, to power on the `GPIOE` peripheral.
- Finally, you'll have to change the starter code to *modify* the right register to turn on the
  `GPIOE` peripheral.

If you are successful, you'll see that the `gpioe.odr.write` statement will now be able to modify
the value of the `ODR` register.

Note that this won't be enough to actually turn on the LEDs.
