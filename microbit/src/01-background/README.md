# Background

## What's a microcontroller?

A microcontroller is a *system* on a chip. Whereas your computer is made up of several discrete
components: a processor, RAM, storage, an Ethernet port, etc.; a microcontroller has all those types
of components built into a single "chip" or package. This makes it possible to build systems with
fewer parts.

## What can you do with a microcontroller?

Lots of things! Microcontrollers are the central part of what are known as "*embedded* systems".
Embedded systems are everywhere, but you don't usually notice them. They control the machines that
wash your clothes, print your documents, and cook your food. Embedded systems keep the buildings
that you live and work in at a comfortable temperature, and control the components that make the
vehicles you travel in stop and go.

Most embedded systems operate without user intervention. Even if they expose a user interface like a
washing machine does; most of their operation is done on their own.

Embedded systems are often used to *control* a physical process. To make this possible, they have
one or more devices to tell them about the state of the world ("sensors"), and one or more
devices which allow them to change things ("actuators"). For example, a building climate control
system might have:

- Sensors which measure temperature and humidity in various locations.
- Actuators which control the speed of fans.
- Actuators which cause heat to be added or removed from the building.

## When should I use a microcontroller?

Many of the embedded systems listed above could be implemented with a computer running Linux (for
example a "Raspberry Pi"). Why use a microcontroller instead? Sounds like it might be harder to
develop a program.

Some reasons might include:

**Cost.** A microcontroller is much cheaper than a general purpose computer. Not only is the
microcontroller cheaper; it also requires many fewer external electrical components to operate.
This makes Printed Circuit Boards (PCB) smaller and cheaper to design and manufacture.

**Power consumption.** Most microcontrollers consume a fraction of the power of a full blown
processor. For applications which run on batteries, that makes a huge difference.

**Responsiveness.** To accomplish their purpose, some embedded systems must always react within a
limited time interval (e.g. the "anti-lock" braking system of a car). If the system misses this
type of *deadline*, a catastrophic failure might occur. Such a deadline is called a "hard real time"
requirement. An embedded system which is bound by such a deadline is referred to as a "hard
real-time system". A general purpose computer and OS usually has many software components which
share the computer's processing resources. This makes it harder to guarantee execution of a program
within tight time constraints.

**Reliability.** In systems with fewer components (both hardware and software), there is less to go
wrong!

## When should I *not* use a microcontroller?

Where heavy computations are involved. To keep their power consumption low, microcontrollers have
very limited computational resources available to them. For example, some microcontrollers don't
even have hardware support for floating point operations. On those devices, performing a simple
addition of single precision numbers can take hundreds of CPU cycles.

## Why use Rust and not C?

Hopefully, I don't need to convince you here as you are probably familiar with the language
differences between Rust and C. One point I do want to bring up is package management. C lacks an
official, widely accepted package management solution whereas Rust has Cargo. This makes development
*much* easier. And, IMO, easy package management encourages code reuse because libraries can be
easily integrated into an application which is also a good thing as libraries get more "battle
testing".

## Why should I not use Rust?

Or why should I prefer C over Rust?

The C ecosystem is way more mature. Off the shelf solutions for several problems already exist. If
you need to control a time sensitive process, you can grab one of the existing commercial Real Time
Operating Systems (RTOS) out there and solve your problem. There are no commercial, production-grade
RTOSes in Rust yet so you would have to either create one yourself or try one of the ones that are
in development. You can find a list of those in the [Awesome Embedded Rust] repository.

[Awesome Embedded Rust]: https://github.com/rust-embedded/awesome-embedded-rust#real-time-operating-system-rtos
