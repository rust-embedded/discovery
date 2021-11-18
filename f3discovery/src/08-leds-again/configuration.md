# Configuration

After turning on the GPIOE peripheral, it still needs to be configured. In this case, we
want the pins to be configured as digital *outputs* so they can drive the LEDs; by default, most
pins are configured as digital *inputs*.

You can find the list of registers in the `GPIOE` register block in:

> Section 11.4.12 - GPIO registers - Page 243 - Reference Manual

The register we'll have to deal with is: `MODER`.

Your task for this section is to further update the starter code to configure the *right* `GPIOE`
pins as digital outputs. You'll have to:

- Figure out *which* pins you need to configure as digital outputs. (hint: check Section 6.4 LEDs of
  the *User Manual* (page 18)).
- Read the documentation to understand what the bits in the `MODER` register do.
- Modify the `MODER` register to configure the pins as digital outputs.

If successful, you'll see the 8 LEDs turn on when you run the program.
