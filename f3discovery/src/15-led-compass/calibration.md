# 校准

如果我们旋转磁强计板，地球磁场相对于磁强计的方向应该会改变，但其大小不应该改变！然而，磁强计显示，磁场的大小随着板的旋转而改变。

为什么会这样？结果磁力计需要校准才能返回正确的答案。

校准涉及相当多的数学（矩阵），因此我们在这里不做介绍，但如果您感兴趣，本[应用说明]将介绍该过程。相反，我们将在本节中做的是*想象*我们的境况。

[应用说明]: https://www.nxp.com/docs/en/application-note/AN4246.pdf

让我们试试这个实验：让我们记录磁强计的读数，同时我们慢慢地向不同的方向旋转磁强计板。
我们将使用`iprintln`宏 将读数格式化为制表符分隔值(TSV)。

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, I16x3};

#[entry]
fn main() -> ! {
    let (_leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();

    loop {
        let I16x3 { x, y, z } = lsm303dlhc.mag().unwrap();

        iprintln!(&mut itm.stim[0], "{}\t{}\t{}", x, y, z);

        delay.delay_ms(100_u8);
    }
}
```

您应该在控制台中获得如下输出：

``` console
$ # itmdump console
-76     213     -54
-76     213     -54
-76     213     -54
-76     213     -54
-73     213     -55
```

您可以使用以下方式将其传输到文件：

``` console
$ # Careful! Exit any running other `itmdump` instance that may be running
$ itmdump -F -f itm.txt > emf.txt
```

在记录数据的同时，沿多个不同方向旋转电路板几秒钟。

然后将该TSV文件导入电子表格程序（或使用下面显示的Python脚本），并将前两列绘制为散点图。

``` python
#!/usr/bin/python

import csv
import math
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
import sys

# apply plot style
sns.set()

x = []
y = []

with open(sys.argv[1], 'r') as f:
    rows = csv.reader(f, delimiter='\t')

    for row in rows:
        # discard rows that are missing data
        if len(row) != 3 or not row[0] or not row[1]:
            continue

        x.append(int(row[0]))
        y.append(int(row[1]))

r = math.ceil(max(max(np.abs(x)), max(np.abs(y))) / 100) * 100

plt.plot(x, y, '.')
plt.xlim(-r, r)
plt.ylim(-r, r)
plt.gca().set_aspect(1)
plt.tight_layout()

plt.savefig('emf.svg')
plt.close
```

<p>
<img title="Earth's magnetic field" src="../assets/emf.svg">
</p>

如果你在一个平坦的水平表面上旋转电路板，磁场的Z分量应该保持相对恒定，这个图应该是以原点为中心的圆周（而不是椭圆）。
如果你以随机的方向旋转棋盘，这就是上面的图，那么你应该得到一个由以原点为中心的一堆点组成的圆。与圆形的偏差表明磁力计需要校准。

Take home message: 不要只相信传感器的读数。验证它是否输出了可感知的值。如果不是，则进行校准。
