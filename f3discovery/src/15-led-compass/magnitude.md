# 大小

我们一直在研究磁场的方向，但它的真实大小是多少？`magnetic_field`函数报告的数量是无单位的。
我们如何将这些值转换为Gauss？

文档将回答这个问题。

> 第2.1节传感器特性-第10页-LSM303DLHC数据表

该页中的表显示了根据GN bits的值具有不同值的*磁增益设置*。默认情况下，这些GN bits设置为`001`。
这意味着X轴和Y轴的磁增益为`1100 LSB / Gauss`，Z轴的磁增量为`980 LSB / Gauss`。LSB代表最低有效位，
`1100 LSB / Gauss`数表示读数`1100`等于`1 Gauss`，读数`2200`等于`2 Gauss`，依此类推。

因此，我们需要做的是将传感器输出的X、Y和Z值除以相应的*增值*。然后，我们将得到磁场的X、Y和Z分量，单位为Gauss。

通过一些额外的数学运算，我们可以从X、Y和Z分量中检索磁场的大小：

``` rust
let magnitude = (x * x + y * y + z * z).sqrt();
```

将所有这些放在一个程序中：

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, I16x3};
use m::Float;

#[entry]
fn main() -> ! {
    const XY_GAIN: f32 = 1100.; // LSB / G
    const Z_GAIN: f32 = 980.; // LSB / G

    let (_leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();

    loop {
        let I16x3 { x, y, z } = lsm303dlhc.mag().unwrap();

        let x = f32::from(x) / XY_GAIN;
        let y = f32::from(y) / XY_GAIN;
        let z = f32::from(z) / Z_GAIN;

        let mag = (x * x + y * y + z * z).sqrt();

        iprintln!(&mut itm.stim[0], "{} mG", mag * 1_000.);

        delay.delay_ms(500_u16);
    }
}
```

该程序将以毫高斯 (`mG`)为单位报告磁场的大小（强度）。地球磁场的大小在`250 mG`到`650 mG`的范围内（大小取决于你的地理位置），所以
你应该看到一个在这个范围内或接近这个范围的值 -- 我看到的大约是210 mG。

一些问题：

不移动开发板，你会看到什么值？你总是看到同样的值吗？

如果你旋转板，幅度会改变吗？它应该改变吗？
