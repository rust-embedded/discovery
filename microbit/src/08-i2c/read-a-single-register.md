# Read a single register

Let's put all that theory into practice!

First things first we need to know the target addresses of both the accelerometer
and the magnetometer inside the chip, these can be found in the LSM303AGR's
datasheet on page 39 and are:

- 0011001 for the accelerometer
- 0011110 for the magnetometer

> **NOTE** Remember that these are only the 7 leading bits of the address,
> the 8th bit is going to be the bit that determines whether we are
> performing a read or write.

Next up we'll need a register to read from. Lots of I2C chips out there will
provide some sort of device identification register for their controllers to read.
This is done since considering the thousands (or even millions) of I2C chips
out there it is highly likely that at some point two chips with the same address
will end up being built (after all the address is "only" 7 bit wide). With
this device ID register a driver could then make sure that it is indeed talking
to a LSM303AGR and not some other chip that just happens to have the same address.
As you can read in the LSM303AGR's datasheet (specifically on page 46 and 61)
it does provide two registers called `WHO_AM_I_A` at address `0x0f` and `WHO_AM_I_M`
at address `0x4f` which contain some bit patterns that are unique to the device
(The A is as in accelerometer and the M is as in magnetometer).

The only thing missing now is the software part, i.e. which API of the `microbit`/the HAL
crates we should use for this. However, if you read through the datasheet of the nRF chip
you are using you will soon find out that they don't actually have an I2C peripheral.
Luckily for us though, they have I2C-compatible ones called TWI (Two Wire Interface)
and TWIM (depending on which chip you use, just like UART and UARTE).

Now if we put the documentation of the [`twi(m)` module] from the `microbit` crate
together with all the other information we have gathered so far we'll end up with this
piece of code to read out and print the two device IDs:

[`twi(m)` module]: https://docs.rs/microbit-v2/0.11.0/microbit/hal/twim/index.html

``` rust
{{#include src/main.rs}}
```

Apart from the initialization, this piece of code should be straight forward if you
understood the I2C protocol as described before. The initialization here works similarly
to the one from the UART chapter.
We pass the peripheral as well as the pins that are used to communicate with the chip to the constructor; and then the frequency we wish the bus to operate on, in this case 100 kHz (`K100`).

## Testing it
As always you have to modify `Embed.toml` to fit your MCU and can then use:
```console
# For micro:bit v2
$ cargo embed --features v2 --target thumbv7em-none-eabihf

# For micro:bit v1
$ cargo embed --features v1 --target thumbv6m-none-eabi
```
in order to test our little example program.
