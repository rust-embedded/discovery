# One-shot timer

I hope that, by now, I have convinced you that `for` loop delays are a poor way
to implement delays.

Now, we'll implement delays using a "timer". The basic function of a timer is
... to keep precise track of time. A timer is yet another peripheral that's
available to the microcontroller thus it can be controlled using registers.

The microcontroller we are using has several (in fact, more than 10) timers of
different kinds (basic, general purpose, and advanced timers) available to it.
Some timers have more "precision" than others and some can be used for more than
just keeping track of time.

We'll be using one of the "basic" timers: `TIM7`. This is one of the simplest
timers available in our microcontroller. The documentation for basic timers is
in the following section:

> Section 22 Timers - Page 674 - Reference Manual

Its registers are documented in:

> Section 22.4.9 TIM6/TIM7 register map - Page 686 - Reference Manual

The registers we'll be using in this section are:

- `SR`, the status register.
- `EGR`, the event generation register.
- `CNT`, the counter register.
- `PSC`, the prescaler register.
- `ARR`, the autoreload register.

We'll be using the timer as a "one-shot" timer. It will sort of work like an
alarm clock. We'll set the timer to "go off" after some amount of time and then
we'll wait until the timer "goes off". The documentation refers to this mode of
operation as "one pulse mode".

Here's a description of how a basic timer works when configured in one pulse
mode:

- The counter is enabled by the user (`CR1.CEN = 1`).
- The `CNT` register resets its value to zero and, on each "tick", its value
  gets incremented by one.
- Once the `CNT` register has reached the value of the `ARR` register, the
  counter will be disabled by hardware (`CR1.CEN = 0`) and an "update" event
  will be raised (`SR.UIF = 1`).

`TIM7` is driven by the APB1 clock, whose frequency doesn't have to necessarily
match the processor frequency. That is, the APB1 clock could be running faster
or slower. The default, however, is that both APB1 and the processor are clocked
at 8 MHz.

The "tick" mentioned in the functional description of the one pulse mode is
*not* the same as one tick of the APB1 clock. The `CNT` register usually works
at a slower rate because this register actually operates at a frequency of
`APB1_CLOCK / (PSC + 1)`, where `PSC` is the value of the prescaler register
(`PSC`).
