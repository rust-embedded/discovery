# Read a single register

Let's put all that theory into practice!

First things first we need to know the slave addresses of both the accelerometer
and the magnetometer inside the chip, these can be found in the LSM303AGR's
datasheet on page 39 and are:

- 0011001 for the accelerometer
- 0011110 for the magnetometer

> **NOTE** Remember that these are only the 7 leading bits of the address,
> the 8th bit is going to be the bit that determines  whether we are
> performing a read or write.

Next up we'll need a register to read from. Lots of I2C chips out there will
provide some sort of device identification register for their masters to read.
This is done since considering the thousands (or even millions) of I2C chips
out there it is highly likely that at some point two chips with the same address
will end up being built (after all the address is "only" 7 bit wide). With
this device ID register a driver could then make sure that it is indeed talking
to a LSM303AGR and not some other chip that just happens to have the same address.
As you can read in the LSM303AGR's datasheet (specifically on page 46 and 61)
it does provide two registers called `WHO_AM_I_A` at address `0x0f` and `WHO_AM_I_M`
at address `0x4f` which contain some bit patterns that are unique to the device
(The A is as in accelerometer and the M is as in magnetometer).

With these two things regarding the slave chip out of the way we'll now have
to take a look at the master. As always we'll have to figure out the way our
microcontroller is connected to the chip we want to communicate with. On page
3 of the [schematic] you'll find the LSM303AGR, as you already know from the
I2C protocol description we are particularly interested in where the SCL and SDA
lines are connected to. On page 5 of the schematic you'll see that SCL is connected
to P0.00 and SDA is connected to P0.30.

[schematic]: https://github.com/bbcmicrobit/hardware/blob/master/V1.5/SCH_BBC-Microbit_V1.5.PDF

The only thing missing now is the software part, i.e. which API of the `nrf51-hal`
we should use for this. However if you take a look at the index of the
[HAL's documentation] you'll notice that nothing named I2C is actually noted
there. This is because some manufacturers don't name their I2C peripheral
"I2C" but instead "TWI" (as in Two Wire Interface), meaning that the TWI module
is the one we are interested in.

[HAL's documentation]: https://docs.rs/nrf51-hal/0.12.1/nrf51_hal/index.html

Now if we put the documentation of this module together will all the other information
we have gathered so far we'll end up with this piece of code to read out and
print the two device IDs:

``` rust
{{#include src/main.rs}}
```

## Testing it
As always you can just use
```
$ cargo embed
```
in order to test our little example program.
