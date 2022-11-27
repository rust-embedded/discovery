# LSM303AGR

Both of the motion sensors on the micro:bit, the magnetometer and the accelerometer, are packaged in a single
component: the LSM303AGR integrated circuit. These two sensors can be accessed via an I2C bus. Each
sensor behaves like an I2C target and has a *different* address.

Each sensor has its own memory where it stores the results of sensing its environment. Our
interaction with these sensors will mainly involve reading their memory.

The memory of these sensors is modeled as byte addressable registers. These sensors can be
configured too; that's done by writing to their registers. So, in a sense, these sensors are very
similar to the peripherals *inside* the microcontroller. The difference is that their registers are
not mapped into the microcontrollers' memory. Instead, their registers have to be accessed via the
I2C bus.

The main source of information about the LSM303AGR is its [Data Sheet]. Read through it to see how
one can read the sensors' registers. That part is in:

[Data Sheet]: https://www.st.com/resource/en/datasheet/lsm303agr.pdf

> Section 6.1.1 I2C Operation - Page 38 - LSM303AGR Data Sheet

The other part of the documentation relevant to this book is the description of the registers. That
part is in:

> Section 8 Register description - Page 46 - LSM303AGR Data Sheet
