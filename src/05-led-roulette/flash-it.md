# Flash it

Flashing is the process of moving our program into the microcontroller's (persistent) memory. Once
flashed, the microcontroller will execute the flashed program every time it is powered on.

In this case, our `led-roulette` program will be the *only* program in the microcontroller memory.
By this I mean that there's nothing else running on the microcontroller: no OS, no "daemon",
nothing. `led-roulette` has full control over the device.

Flashing the binary itself is quite simple thanks to `cargo-embed`, you only have to type `cargo-embed`.

Before executing that command though, lets look into what it actually does. If you look at the side of your micro:bit
with the USB connector facing upwards you will notice, that there are actually 2 black squares on there, one is our MCU
we already talked about but what purpose does the other one server? The other chip has 3 main purposes:

1. Provide power from the USB connector to our MCU
2. Provide a serial to USB bridge for our MCU (we will look into that in a later chapter)
3. Being a programmer/debugger (this is the relevant purpose for now)

Basically this chip acts as a bridge between our computer (to which it is connected via USB) and the MCU (to which it is
connected via traces and communicates with using the SWD protocol), this bridge enables us to flash new binaries on to
the MCU, inspect the state of the MCU via a debugger and other things.

So lets flash it!

```console
$ cargo-embed
  (...)
        WARN probe_rs::config::registry > Found chip nRF51822_xxAA which matches given partial name nrf51822. Consider specifying its full name.
        WARN probe_rs::config::registry > Found chip nRF51822_xxAB which matches given partial name nrf51822. Consider specifying its full name.
        WARN probe_rs::config::registry > Found chip nRF51822_xxAC which matches given partial name nrf51822. Consider specifying its full name.
     Erasing sectors ✔ [00:00:00] [##################################################################################################################################################################]   2.00KB/  2.00KB @   4.57KB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [##################################################################################################################################################################]   2.00KB/  2.00KB @   1.93KB/s (eta 0s )
    Finished flashing in 0.764s
Firing up GDB stub at localhost:1337.
GDB stub listening on localhost:1337
```

> **NOTE**: The warnings it printed are irrelevant for us, it is just warning us that there are several nRF51822 variants and we
> might not have picked the right one (which we obviously did though, as the flashing succeeded).

You will notice that `cargo-embed` blocks after outputting the last line, this is inteded and you should not close it
since we need it in this state for the next step, debugging it!
