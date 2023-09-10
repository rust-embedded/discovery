# Take 2

这一次，我们将使用数学来获得磁场与磁力计X轴和Y轴形成的精确角度。

我们将使用`atan2`函数。此函数返回`-PI`到`PI`范围内的角度。下图显示了如何测量该角度：

<p>
<img class="white_bg" title="atan2" src="https://upload.wikimedia.org/wikipedia/commons/0/03/Atan2_60.svg">
</p>

虽然未在该图中明确显示，但X轴指向右侧，Y轴指向上方。

这是启动码。`theta`弧度已经计算出来了。您需要根据`theta`的值选择要打开哪个LED。

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

// You'll find this useful ;-)
use core::f32::consts::PI;

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, switch_hal::OutputSwitch, Direction, I16x3};
// this trait provides the `atan2` method
use m::Float;

#[entry]
fn main() -> ! {
    let (leds, mut lsm303dlhc, mut delay, _itm) = aux15::init();
    let mut leds = leds.into_array();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let _theta = (y as f32).atan2(x as f32); // in radians

        // FIXME pick a direction to point to based on `theta`
        let dir = Direction::Southeast;

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(100_u8);
    }
}
```

建议/提示：

- 整圈旋转等于360度。
- `PI`弧度等于180度。
- 如果 `theta` 为零，你会打开什么LED？
- 如果 `theta` 非常接近零，你会打开什么LED？
- 如果 `theta` 持续增加，你会以什么值打开不同的LED？
