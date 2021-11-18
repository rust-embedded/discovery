# LSM303DLHC

\* **NOTE**: Newer (from around 2020/09) Discovery boards may have an [LSM303AGR][agr]
rather than a [LSM303DLHC][Data Sheet]. 
Checkout the github issues like [this][gh-issue-274] for more details. 

[agr]: https://www.st.com/resource/en/datasheet/lsm303agr.pdf
[gh-issue-274]: https://github.com/rust-embedded/discovery/issues/274

Two of the sensors in the F3, the magnetometer and the accelerometer, are packaged in a single
component: the LSM303DLHC integrated circuit. These two sensors can be accessed via an I2C bus. Each
sensor behaves like an I2C slave and has a *different* address.

Each sensor has its own memory where it stores the results of sensing its environment. Our
interaction with these sensors will mainly involve reading their memory.

The memory of these sensors is modeled as byte addressable registers. These sensors can be
configured too; that's done by writing to their registers. So, in a sense, these sensors are very
similar to the peripherals *inside* the microcontroller. The difference is that their registers are
not mapped into the microcontrollers' memory. Instead, their registers have to be accessed via the
I2C bus.

The main source of information about the LSM303DLHC is its [Data Sheet]. Read through it to see how
one can read the sensors' registers. That part is in:

[Data Sheet]: http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf

> Section 5.1.1 I2C Operation - Page 20 - LSM303DLHC Data Sheet

The other part of the documentation relevant to this book is the description of the registers. That
part is in:

> Section 7 Register description - Page 25 - LSM303DLHC Data Sheet
