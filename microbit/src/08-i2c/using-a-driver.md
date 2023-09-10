# 使用驱动程序

正如我们在第5章中已经讨论过的，`embedded-hal`提供了一些抽象，可以用来编写与硬件交互的平台无关代码。
事实上， 我们在第7章和到目前为止在第8章中用于与硬件交互的所有方法都来自于由`embedded-hal`定义的特征。
现在，我们将首次实际使用`embedded-hal`提供的特性。

为我们的LSM303AGR为每个嵌入式Rust支持的平台（以及可能最终弹出的新平台）实现驱动程序是没有意义的。
为了避免这种情况，可以编写使用实现`embedded-hal`特征的泛型类型的驱动程序，以提供驱动程序的平台无关版本。
幸运的是，这已经在[`lsm303agr`]crate中完成了。因此， 读取实际加速度计和磁强计值现在基本上是一种即插即用体验（再加上阅读一些文档）。
事实上是`crates.io`页面已经为我们提供了读取加速度计数据所需的所有信息，但使用的是Raspberry Pi。我们只需要将它适应我们的芯片：

[`lsm303agr`]: https://crates.io/crates/lsm303agr

```rust
use linux_embedded_hal::I2cdev;
use lsm303agr::{AccelOutputDataRate, Lsm303agr};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Lsm303agr::new_with_i2c(dev);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            println!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
        }
    }
}
```

因为我们已经知道如何创建实现[上一页](read-a-single-register.md)中[`embedded_hal::blocking::i2c`]特性的对象实例，所以这非常简单：

[`embedded_hal::blocking::i2c`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/blocking/i2c/index.html

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

#[cfg(feature = "v1")]
use microbit::{
    hal::twi,
    pac::twi0::frequency::FREQUENCY_A,
};

#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{
    AccelOutputDataRate, Lsm303agr,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();


    #[cfg(feature = "v1")]
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    // Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            // RTT instead of normal print
            rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
        }
    }
}
```

就像最后一个片段一样，您应该可以这样尝试：
```console
# For micro:bit v2
$ cargo embed --features v2 --target thumbv7em-none-eabihf

# For micro:bit v1
$ cargo embed --features v1 --target thumbv6m-none-eabi
```

此外，如果您（物理上）在您的micro:bit周围移动一点，您应该会看到正在打印的加速度数字发生了变化。
