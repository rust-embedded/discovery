# [Rust Belt Rust 2016]

[Rust Belt Rust 2016]: http://www.rust-belt-rust.com/

> [Workshop] Bare metal: Programming ARM microcontrollers in Rust

## What's a microcontroller?

A microcontroller is a *system* on a chip. Whereas your laptop is made up of
several discrete components: a processor, RAM sticks, a hard drive, an ethernet
port, etc.; a microcontroller has all those components built into a single
"chip" or package. This makes it possible to build systems with minimal part
count.

## What can you do with a microcontroller?

Lots of things! Microcontrollers are the central part of systems known as
*embedded* systems. These systems are everywhere but you can't usually tell that
they are there. These systems control the brakes of your car, wash your clothes,
print your papers, keep you warm, keep you cool, optimize fuel consumption, etc.

The main trait of these systems is that they operate without user intervention
even if they expose a user interface like a washing machine does most of their
operation is done on their own.

The other common trait of these systems is that they *control* a process. And
for that these systems usually have one or more sensors and one or more
actuators. For example, an HVAC system has several sensors, thermometers and
humidy sensors spread across some area, and several actuators as well, heating
elements and fans connected to ducts.

## When should I use a microcontroller?

All these application I've mentioned, you can probably implement with a
Raspberry Pi, a computer that runs Linux. Why should I bother with a
microcontroller that operates without an OS? Sounds like it would be harder to
develop a program.

The main reason is cost. A microcontroller is much cheaper than a general
purpose computer. Not only the microcontroller is cheaper; it also requires much
less external electrical components to operate. This makes Printed Circuit
Boards (PCB) smaller and cheaper to design and manufacture.

The other big reason is power consumption. A microcontroller consumes orders of
magnitude less power than a full blown processor. If your application will
run on batteries that makes a huge difference.

At last but not least: precise timing control. A general purpose computer
running a general purpose OS has many services running in the background. This
makes it hard to guarantee execution of a program within tight time constraints
(microsecond range for instance).

## When should I *not* use a microcontroller?

Where heavy computations are involved. To keep their power consumption low,
microcontrollers have very limited computational resources available to them.
For example, some microcontrollers don't even have hardware support for floating
point operations. On those devices, performing a simple addition of single
precision numbers can take hundreds of CPU cycles.

## Why use Rust and not C?

Hopefully, I don't need to convince you here as you are probably familiar with
the language differences between Rust and C. One point I do want to bring up is
package management. C lacks an official, widely accepted package management
solution whereas Rust has Cargo. This makes development *much* easier. And, IMO,
easy package management encourages code reuse because libraries can be easily
integrated into an application which is a good thing as libraries get more
"battle testing".

## Why should I not use Rust?

Or why should I prefer C over Rust?

The C ecosystem is way more mature. Off the shelf solution for several problems
already exist. If you need to control a time sensitive process, you can grab one
of the existing commercial Real Time Operating Systems (RTOS) out there and
solve your problem. There are no commercial, production-grade RTOSes in Rust yet
so you would have to either create one yourself or try one of the ones that are
in development.

---

Enough marketing. Let's actually build stuff!

## The hardware

On the day of the workshop, we'll provide you this hardware:

- A [STM32F3DISCOVERY] board.

[STM32F3DISCOVERY]: http://www.st.com/en/evaluation-tools/stm32f3discovery.html

<p align="center">
<img title="STM32F3DISCOVERY" src="assets/f3.jpg">
</p>

- [This **3.3V** USB <-> Serial module][sparkfun] (\*)

[sparkfun]: https://www.sparkfun.com/products/9873

(If you are reading this after the workshop, you can follow this material if you
have a different model.)

<p align="center">
<img title="A 3.3v USB <-> Serial module" src="assets/serial.jpg">
</p>

- A HC-05 Bluetooth module (with headers!)

<p align="center">
<img title="The HC-05 Bluetooth module" src="assets/bluetooth.jpg">
</p>

- Two mini-B USB cables (preferably at least 50cm (~2ft) long).

<p align="center">
<img title="mini-B USB cable" src="assets/usb-cable.jpg">
</p>

> **NOTE** These are **not** the USB cables that ship with pretty much every
> Android phone; those are *micro* USB cables. Make sure you have the right
> thing!

- 4 Female/Female, 4 Male/Female and 1 Male/Male jumper wires

<p align="center">
<img title="Jumper wires" src="assets/jumper-wires.jpg">
</p>

## Documentation

This is the documentation we'll refer to during the workshop.

- [STM32F3DISCOVERY User Manual][um]
- [STM32F303VC Datasheet][ds]
- [STM32F303VC Reference Manual][rm]
- [LSM303DLHC]
- [L3GD20]

[L3GD20]: http://www.st.com/resource/en/datasheet/l3gd20.pdf
[LSM303DLHC]: http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf
[ds]: http://www.st.com/resource/en/datasheet/stm32f303vc.pdf
[rm]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf
[um]: http://www.st.com/resource/en/user_manual/dm00063382.pdf
