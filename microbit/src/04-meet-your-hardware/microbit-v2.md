# Nordic nRF52833 (the "nRF52", micro:bit v2)

Our MCU has 73 tiny metal **pins** sitting right underneath it (it's a so called [aQFN73] chip).
These pins are connected to **traces**, the little "roads" that act as the wires connecting components
together on the board. The MCU can dynamically alter the electrical properties
of the pins. This works similar to a light switch altering how electrical
current flows through a circuit. By enabling or disabling electrical current to
flow through a specific pin, an LED attached to that pin (via the traces) can
be turned on and off.

Each manufacturer uses a different part numbering scheme, but many will allow
you to determine information about a component simply by looking at the part
number. Looking at our MCU's part number (`N52833 QIAAA0 2024AL`, you probably cannot
see it with your bare eye, but it is on the chip), the `n` at the
front hints to us that this is a part manufactured by [Nordic Semiconductor].
Looking up the part number on their website we quickly find the [product page].
There we learn that our chip's main marketing point is that it is a
"Bluetooth Low Energy and 2.4 GHz SoC" (SoC being short for "System on a Chip"),
which explains the RF in the product name since RF is short for radio frequency.
If we search through the documentation of the chip linked on the [product page]
for a bit we find the [product specification] which contains chapter 10 "Ordering Information"
dedicated to explaining the weird chip naming. Here we learn that:

[aQFN73]: https://en.wikipedia.org/wiki/Flat_no-leads_package
[Nordic Semiconductor]: https://www.nordicsemi.com/
[product page]: https://www.nordicsemi.com/products/nrf52833
[product specification]: https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.3.pdf

- The `N52` is the MCU's series, indicating that there are other `nRF52` MCUs
- The `833` is the part code
- The `QI` is the package code, short for `aQFN73`
- The `AA` is the variant code, indicating how much RAM and flash memory the MCU has,
  in our case 512 kilobyte flash and 128 kilobyte RAM
- The `A0` is the build code, indicating the hardware version (`A`) as well as the product configuration (`0`)
- The `2024AL` is a tracking code, hence it might differ on your chip

The product specification does of course contain a lot more useful information about
the chip, for example that it is based on an ARM® Cortex™-M4 32-bit processor.


## Arm? Cortex-M4?

If our chip is manufactured by Nordic, then who is Arm? And if our chip is the
nRF52833, what is the Cortex-M4?

You might be surprised to hear that while "Arm-based" chips are quite
popular, the company behind the "Arm" trademark ([Arm Holdings]) doesn't
actually manufacture chips for purchase. Instead, their primary business
model is to just *design* parts of chips. They will then license those designs to
manufacturers, who will in turn implement the designs (perhaps with some of
their own tweaks) in the form of physical hardware that can then be sold.
Arm's strategy here is different from companies like Intel, which both
designs *and* manufactures their chips.

Arm licenses a bunch of different designs. Their "Cortex-M" family of designs
are mainly used as the core in microcontrollers. For example, the Cortex-M4
(the core our chip is based on) is designed for low cost and low power usage.
The Cortex-M7 is higher cost, but with more features and performance.

Luckily, you don't need to know too much about different types of processors
or Cortex designs for the sake of this book. However, you are hopefully now a
bit more knowledgeable about the terminology of your device. While you are
working specifically with an nRF52833, you might find yourself reading
documentation and using tools for Cortex-M-based chips, as the nRF52833 is
based on a Cortex-M design.

[Arm Holdings]: https://www.arm.com/
