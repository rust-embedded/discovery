<!-- # Calibration -->

# キャリブレーション

<!-- 
If we rotate the board, the direction of the Earth's magnetic field with respect to the magnetometer
should change but its magnitude should not! Yet, the magnetometer indicates that the magnitude of
the magnetic field changes as the board rotates.
 -->

ボードを回転すると、磁力計に対する地球磁場の方向は変わりますが、大きさは変わりません！
それでも、磁力計は、ボードが回転すると磁場の大きさが変化することを示しています。

<!-- 
Why's that the case? Turns out the magnetometer needs to be calibrated to return the correct answer.
 -->

それはなぜでしょうか？結局のところ、磁力計が正しい答えを返すためには、キャリブレーションが必要なのです。

<!-- 
The calibration involves quite a bit of math (matrices) so we won't cover it here but this
[Application Note] describes the procedure if you are interested. Instead, what we'll do in this
section is *visualize* how off we are.
 -->

キャリブレーションには、多くの数学（行列）が必要であるため、ここでは取り上げません。もし興味があるのであれば、
[アプリケーションノート]にその計算方法が記述されています。
代わりに、このセクションでやることは、どの程度正しい答えからずれているか、を*視覚化*することです。

<!-- [Application Note]: http://cache.freescale.com/files/sensors/doc/app_note/AN4246.pdf -->

[アプリケーションノート]: http://cache.freescale.com/files/sensors/doc/app_note/AN4246.pdf

<!-- 
Let's try this experiment: Let's record the readings of the magnetometer while we slowly rotate the
board in different directions. We'll use the `iprintln` macro to format the readings as Tab
Separated Values (TSV).
 -->

次の実験を試してみましょう。別方向にボードをゆっくり回転している間、磁力計から読んだ値を記録しましょう。
読んだ値をタブ区切り（TSV; Tab Separated Values）に変換するため、`iprintln`マクロを使います。

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

<!-- You should get an output in the console that looks like this: -->

コンソールに次のような出力が得られるはずです。
``` console
$ # itmdumpコンソール
-76     213     -54
-76     213     -54
-76     213     -54
-76     213     -54
-73     213     -55
```

<!-- You can pipe that to a file using: -->

次のコマンドを使って、ファイルにパイプできます。

``` console
$ # 注意！他に実行中の`itmdump`インスタンスを全て終了します
$ itmdump -F -f itm.txt > emf.txt
```

<!-- Rotate the board in many different direction while you log data for a several seconds. -->

数秒間、データをログ出力している間、ボードを様々な方向に回転します。

<!-- 
Then import that TSV file into a spreadsheet program (or use the Python script shown below) and plot
the first two columns as a scatter plot.
 -->

その後、TSVファイルをスプレッドシートプログラムに取り込み（もしくは、下記のPythonスクリプトを使い）、
最初の2つの列を散布図としてプロットします。

``` python
#!/usr/bin/python

import csv
import math
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
import sys

# プロット形式を適用します
sns.set()

x = []
y = []

with open(sys.argv[1], 'r') as f:
    rows = csv.reader(f, delimiter='\t')

    for row in rows:
        # データが欠けている行は捨てます
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

<p align="center">
<img title="Earth's magnetic field" src="../assets/emf.svg">
</p>

<!-- 
If you rotated the board on a flat horizontal surface, the Z component of the magnetic field should
have remained relatively constant and this plot should have been a circumference (not a ellipse)
centered at the origin. If you rotated the board in random directions, which was the case of plot
above, then you should have gotten a circle made of a bunch of points centered at the origin.
Deviations from the circle shape indicate that the magnetometer needs to be calibrated.
 -->

ボードを水平な平面上で回転した場合、磁場のZ要素は、比較的同じ値を取り続けたはずです。
このプロットは、原点を中心とした円形（楕円形でない）でなければなりません。
上図のプロットのように、ボードをランダムな方向に回転した場合、多数の点からなる原点を中心とした円が得られるはずです。
円形からの偏差は、磁力計をキャリブレーションする必要があることを示しています。

<!-- 
Take home message: Don't just trust the reading of a sensor. Verify it's outputting sensible values.
If it's not, then calibrate it.
 -->

覚えておいてほしいこと：センサから読んだ値を単純に信用しないで下さい。適切な値が出力されていることを確認して下さい。
適切な出力でなければ、キャリブレーションして下さい。
