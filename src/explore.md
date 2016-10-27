# What's left for you to explore

We have barely scratched the surface!

## Direct Memory Access (DMA).

This peripheral is a kind of *asynchronous* `memcpy`. So far our programs have
been pumping data, byte by byte, into peripherals like UART and I2C. This DMA
peripheral can be used to perform bulk transfers of data. Either from RAM to
RAM, from a peripheral, like a UART, to RAM or from RAM to a peripheral.


## Sleeping

All our programs have been continuously polling peripherals to see if there's
anything that needs to be done. However, some times there's nothing to be done!
At those times, the microcontroller should "sleep".

When the processor sleeps, it stops executing instructions and this saves power.
It's almost always a good to save power so your microcontroller should be
sleeping most of the time. But, how does it know when it has to wake up to
perform some action? Interrupts are one of the events that wake up the
microcontroller but there are others.

Keywords: The `wfi` and `wfe` instructions.

## Pulse Width Modulation (PWM)

Controlling "power".

## Digital input

Integrating buttons and switches into your application.

## Sensor fusion

The STM32F3DISCOVERY contains

## Analog-to-Digital Converters (ADC)

There are a lots of digital sensors out there. You can use a protocol like I2C
and SPI to read them. But analog sensors also exist! These sensors just output a
voltage level that's proportional to the magnitude they are sensing.

The ADC peripheral can be use to convert that "analog" voltage level, say `1.25`
Volts,into a "digital" number, say in the `[0, 65535]` range, that the processor
can use in its calculations.

## Digital-to-Analog Converters (DAC)

## Real Time Clock (RTC)

This peripheral can be used to track time in "human format". Seconds, minutes,
hours, days, months and years. This peripheral handles the translation from
"ticks" to these human friendly units of time. It even handles leap years and
Daylight Save Time for you!

## Other communication protocols

I2S, SMBUS, CAN, IrDA, Ethernet, USB, etc.
