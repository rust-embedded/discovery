# Registers

It's time to explore what the `Led` API does under the hood.

In a nutshell, it just writes to some special memory regions. Go into the
`07-registers` directory and let's run the starter code.

``` rust
#![no_std]
#![no_main]

extern crate pg;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        // Turn on the "North" LED (red)
        *(GPIOE_BSRR as *mut u32) = 1 << 9;

        // Turn on the "East" LED (green)
        *(GPIOE_BSRR as *mut u32) = 1 << 11;

        // Turn off the "North" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

        // Turn off the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
    }

    loop {}
}
```

What's this magic?

The address `0x48001018` points to a *register*. A register is special region
of memory that controls a *peripheral*. A peripheral is a piece of electronics
that sits right next to the processor within the microcontroller package and
provides the processor extra functionality. After all, the processor, on its
own, can only do math and logic.

This particular register controls General Purpose Input/Output (GPIO) *pins*
(GPIO *is* a peripheral) and can be used to *drive* each of those pins *low* or
*high*.

## An aside: LEDs, digital outputs and voltage levels

Drive? Pin? Low? High?

A pin is a electrical contact. Our microcontroller has several of them and some
of them are connected to LEDs. An LED, a Light Emitting Diode, will only emit
light when voltage is applied to it with a certain polarity.

<p align="center">
<img height=180 title="LED circuit" src="https://upload.wikimedia.org/wikipedia/commons/c/c9/LED_circuit.svg">
</p>

Luckily for us, the microcontroller's pins are connected to the LEDs with the
right polarity. All that we have to do is *output* some non-zero voltage through
the pin to turn the LED on. The pins attached to the LEDs are configured as
*digital outputs* and can only output two different voltage levels: "low", 0
Volts, or "high", 3 Volts. A "high" (voltage) level will turn the LED on whereas
a "low" (voltage) level will turn it off.

These "low" and "high" states map directly to the concept of digital logic.
"low" is `0` or `false` and "high" is `1` or `true`. This is why this pin
configuration is known as digital output.

---

OK. But how can one find out what this register does? Time to RTRM!
