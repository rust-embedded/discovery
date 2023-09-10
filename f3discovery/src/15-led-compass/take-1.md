# Take 1

我们实现LED指南针的最简单方法是什么？即使它不完美。

首先，我们只关心磁场的X和Y分量，因为当你看指南针时，你总是把它保持在水平位置，因此指南针在XY平面上。

例如，在以下情况下，您将打开哪个LED。EMF代表地球磁场，绿色箭头表示EMF的方向（它指向北方）。

<p>
<img title="Quadrant I" src="../assets/quadrant-i.png">
</p>

`Southeast` LED，对吗？

在这种情况下，磁场的X和Y分量有什么*迹象*？两者都是正的。

如果我们只看X和Y分量的符号，我们就能确定磁场属于哪个象限。

<p>
<img class="white_bg" title="Quadrants" src="../assets/quadrants.png">
</p>

在前面的示例中，磁场位于第一象限（x和y为正），打开`SouthEast`LED是有意义的。
同样，如果磁场在不同的象限，我们可以打开不同的LED。

让我们试试这个逻辑。以下是启动代码：

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, switch_hal::OutputSwitch, Direction, I16x3};

#[entry]
fn main() -> ! {
    let (leds, mut lsm303dlhc, mut delay, _itm) = aux15::init();
    let mut leds = leds.into_array();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        // Look at the signs of the X and Y components to determine in which
        // quadrant the magnetic field is
        let dir = match (x > 0, y > 0) {
            // Quadrant ???
            (true, true) => Direction::Southeast,
            // Quadrant ???
            (false, true) => panic!("TODO"),
            // Quadrant ???
            (false, false) => panic!("TODO"),
            // Quadrant ???
            (true, false) => panic!("TODO"),
        };

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(1_000_u16);
    }
}
```

`led`模块中有一个`Direction`枚举，它有8个以基点命名的变体：`North`, `East`, `Southwest`等。
每个变体代表指南针中8个led中的一个。`Leds`值可以使用`Direction` `enum`进行索引；索引的结果是指向该`Direction`的LED。
