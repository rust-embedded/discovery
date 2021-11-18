# Meet your hardware

Let's get familiar with the hardware we'll be working with.

## STM32F3DISCOVERY (the "F3")

<p align="center">
<img title="F3" src="../assets/f3.jpg">
</p>

We'll refer to this board as "F3" throughout this book. Here are some of the
many components on the board:

- A [microcontroller].
- A number of LEDs, including the eight aligned in a "compass" formation.
- Two buttons.
- Two USB ports.
- An [accelerometer].
- A [magnetometer].
- A [gyroscope].

[microcontroller]: https://en.wikipedia.org/wiki/Microcontroller
[accelerometer]: https://en.wikipedia.org/wiki/Accelerometer
[magnetometer]: https://en.wikipedia.org/wiki/Magnetometer
[gyroscope]: https://en.wikipedia.org/wiki/Gyroscope

Of these components, the most important is the microcontroller (sometimes
shortened to "MCU" for "microcontroller unit"), which is the large black square
sitting in the center of your board. The MCU is what runs your code. You might
sometimes read about "programming a board", when in reality what we are doing
is programming the MCU that is installed on the board.

## STM32F303VCT6 (the "STM32F3")

Since the MCU is so important, let's take a closer look at the one sitting on our board.

Our MCU is surrounded by 100 tiny metal **pins**. These pins are connected to
**traces**, the little "roads" that act as the wires connecting components
together on the board. The MCU can dynamically alter the electrical properties
of the pins. This works similar to a light switch altering how electrical
current flows through a circuit. By enabling or disabling electrical current to
flow through a specific pin, an LED attached to that pin (via the traces) can
be turned on and off.

Each manufacturer uses a different part numbering scheme, but many will allow
you to determine information about a component simply by looking at the part
number. Looking at our MCU's part number (`STM32F303VCT6`), the `ST` at the
front hints to us that this is a part manufactured by [ST Microelectronics].
Searching through [ST's marketing materials] we can also learn the following:

[ST Microelectronics]: https://st.com/
[ST's marketing materials]: https://www.st.com/en/microcontrollers-microprocessors/stm32-mainstream-mcus.html

- The `M32` represents that this is an Arm®-based 32-bit microcontroller.
- The `F3` represents that the MCU is from ST's "STM32F3" series. This is a
  series of MCUs based on the Cortex®-M4 processor design.
- The remainder of the part number goes into more details about things like
  extra features and RAM size, which at this point we're less concerned about.

> ### Arm? Cortex-M4?
>
> If our chip is manufactured by ST, then who is Arm? And if our chip is the
> STM32F3, what is the Cortex-M4?
>
> You might be surprised to hear that while "Arm-based" chips are quite
> popular, the company behind the "Arm" trademark ([Arm Holdings][]) doesn't
> actually manufacture chips for purchase. Instead, their primary business
> model is to just *design* parts of chips. They will then license those designs to
> manufacturers, who will in turn implement the designs (perhaps with some of
> their own tweaks) in the form of physical hardware that can then be sold.
> Arm's strategy here is different from companies like Intel, which both
> designs *and* manufactures their chips.
>
> Arm licenses a bunch of different designs. Their "Cortex-M" family of designs
> are mainly used as the core in microcontrollers. For example, the Cortex-M0
> is designed for low cost and low power usage. The Cortex-M7 is higher cost,
> but with more features and performance. The core of our STM32F3 is based on
> the Cortex-M4, which is in the middle: more features and performance than the
> Cortex-M0, but less expensive than the Cortex-M7.
>
> Luckily, you don't need to know too much about different types of processors
> or Cortex designs for the sake of this book. However, you are hopefully now a
> bit more knowledgeable about the terminology of your device. While you are
> working specifically with an STM32F3, you might find yourself reading
> documentation and using tools for Cortex-M-based chips, as the STM32F3 is
> based on a Cortex-M design.

[Arm Holdings]: https://www.arm.com/

## The Serial module

<p align="center">
<img title="Serial module" src="../assets/serial.jpg">
</p>

If you have an older revision of the discovery board, you can use this module to
exchange data between the microcontroller in the F3 and your computer. This module
will be connected to your computer using an USB cable. I won't say more at this
point.

If you have a newer release of the board then you don't need this module. The
ST-LINK will double as a USB<->serial converter connected to the microcontroller USART1 at pins PC4 and PC5.

## The Bluetooth module

<p align="center">
<img title="The HC-05 Bluetooth module" src="../assets/bluetooth.jpg">
</p>

This module has the exact same purpose as the serial module but it sends the data over Bluetooth
instead of over USB.
