# Meet your hardware

Let's get familiar with the hardware we'll be working with.

## micro:bit

<p align="center">
<img title="micro:bit" src="../assets/microbit-v2.jpg">
</p>

Here are some of the many components on the board:

- A [microcontroller].
- A number of LEDs, most notably the LED matrix on the back
- Two user buttons as well as a reset button (the one next to the USB port).
- One USB port.
- A sensor that is both a [magnetometer] and an [accelerometer]

[microcontroller]: https://en.wikipedia.org/wiki/Microcontroller
[accelerometer]: https://en.wikipedia.org/wiki/Accelerometer
[magnetometer]: https://en.wikipedia.org/wiki/Magnetometer
[gyroscope]: https://en.wikipedia.org/wiki/Gyroscope

Of these components, the most important is the microcontroller (sometimes
shortened to "MCU" for "microcontroller unit"), which is the bigger of the two
black squares sitting on the side of the board with the USB port. The MCU is
what runs your code. You might sometimes read about "programming a board", when
in reality what we are doing is programming the MCU that is installed on the board.

If you happen to be interested in a more in detail description of the board you
can checkout the [micro:bit website](https://tech.microbit.org/hardware/).

## Nordic nRF51822 (the "nRF51")

Since the MCU is so important, let's take a closer look at the one sitting on our board.

Our MCU has 48 tiny metal **pins** sitting right underneath it (it's a so called [QFN48] chip).
These pins are connected to **traces**, the little "roads" that act as the wires connecting components
together on the board. The MCU can dynamically alter the electrical properties
of the pins. This works similar to a light switch altering how electrical
current flows through a circuit. By enabling or disabling electrical current to
flow through a specific pin, an LED attached to that pin (via the traces) can
be turned on and off.

Each manufacturer uses a different part numbering scheme, but many will allow
you to determine information about a component simply by looking at the part
number. Looking at our MCU's part number (`nRF51822-QFAA-R`, you probably cannot
see it with your bare eye, but it is on the chip), the `n` at the
front hints to us that this is a part manufactured by [Nordic Semiconductor].
Looking up the part number on their website we quickly find the [product page].
There we learn that our chip's main marketing point is that it is a
"Bluetooth Low Energy and 2.4 GHz SoC" (SoC being short for "System on a Chip"),
which explains the RF in the product name since RF is short for radio frequency.
If we search through the documentation of the chip linked on the [product page]
for a bit we find the [product specification] which contains chapter 10 "Ordering Information"
dedicated to explaining the weird chip naming. Here we learn that:

[QFN48]: https://en.wikipedia.org/wiki/Flat_no-leads_package
[Nordic Semiconductor]: https://www.nordicsemi.com/
[product page]: https://www.nordicsemi.com/Products/Low-power-short-range-wireless/nRF51822
[product specification]: https://infocenter.nordicsemi.com/pdf/nRF51822_PS_v3.3.pdf

- The `nRF51` is the MCU's series, indicating that there are other `nRF51` MCUs
- The `822` is the part code
- The `QF` is short for `QFN48`
- The `AA` is the variant code, indicating how much RAM and flash memory the MCU has,
  in our case 256 kilobyte flash and 16 kilobyte RAM
- The `R` is the packaging code which is relevant for factories manufacturing boards
  with this chip on them in larger scales

The product specification does of course contain a lot more useful information about
the chip, for example that it is based on an ARM® Cortex™-M0 32 bit processor.

### Arm? Cortex-M0?

If our chip is manufactured by Nordic, then who is Arm? And if our chip is the
nRF51822, what is the Cortex-M0?

You might be surprised to hear that while "Arm-based" chips are quite
popular, the company behind the "Arm" trademark ([Arm Holdings][]) doesn't
actually manufacture chips for purchase. Instead, their primary business
model is to just *design* parts of chips. They will then license those designs to
manufacturers, who will in turn implement the designs (perhaps with some of
their own tweaks) in the form of physical hardware that can then be sold.
Arm's strategy here is different from companies like Intel, which both
designs *and* manufactures their chips.

Arm licenses a bunch of different designs. Their "Cortex-M" family of designs
are mainly used as the core in microcontrollers. For example, the Cortex-M0
(the core our chip is based on) is designed for low cost and low power usage.
The Cortex-M7 is higher cost, but with more features and performance.

Luckily, you don't need to know too much about different types of processors
or Cortex designs for the sake of this book. However, you are hopefully now a
bit more knowledgeable about the terminology of your device. While you are
working specifically with an nRF51822, you might find yourself reading
documentation and using tools for Cortex-M-based chips, as the nRF51822 is
based on a Cortex-M design.

[Arm Holdings]: https://www.arm.com/
