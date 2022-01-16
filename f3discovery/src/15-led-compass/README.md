<!-- # LED compass -->

# LEDコンパス

<!-- 
In this section, we'll implement a compass using the LEDs on the F3. Like proper compasses, our LED
compass must point north somehow. It will do that by turning on one of its eight LEDs; the on LED
should point towards north.
 -->

このセクションでは、F3のLEDを使ったコンパスを実装します。通常のコンパスのように、LEDコンパスは、どうにかして北を向かなければなりません。
これは、8個のLEDの1つを点灯することで達成します。点灯しているLEDが北の方法を指します。

<!-- 
Magnetic fields have both a magnitude, measured in Gauss or Teslas, and a *direction*. The
magnetometer on the F3 measures both the magnitude and the direction of an external magnetic field
but it reports back the *decomposition* of said field along *its axes*.
 -->

磁場は、ガウスまたはテスラで測定された大きさと、*方向*の両方を持ちます。
F3の磁力計は、外部磁界の大きさと方向とを測定します。しかし、*ボードの軸*に沿って磁場を*分解*したものを報告します。

<!-- See below, the magnetometer has three axes associated to it. -->

下記の通り、磁力計は関連する3つの軸を持ちます。

<p align="center">
<img height=480 title="Magnetometer axes" src="../assets/f3-lsm303dlhc.png">
</p>

<!-- Only the X and Y axes are shown above. The Z axis is pointing "out" of your screen. -->

上図では、XとY軸だけが描かれています。Z軸は、スクリーンの「外」を向いています。

<!-- 
Let's get familiar with the readings of the magnetometer by running the following starter code:
 -->

次のスターターコードを実行して、磁力計の読み取り方に詳しくなりましょう。

``` rust
{{#include src/main.rs}}
```

<!-- 
This `lsm303dlhc` module provides high level API over the LSM303DLHC. Under the hood it does the
same I2C routine that you implemented in the last section but it reports the X, Y and Z values in a
`I16x3` struct instead of a tuple.
 -->

この`lsm303dlhc`モジュールは、LSM303DLHCの高レベルなAPIを提供します。内部では、前回のセクションで実装したI2Cルーチンと同じことをやっています。
しかし、このモジュールは、タプルの代わりに`I16x3`構造体でX、Y、Zの値を報告します。

<!-- 
Locate where north is at your current location. Then rotate the board such that it's aligned
"towards north": the North LED (LD3) should be pointing towards north.
 -->

現在地の北がどこにあるかを探します。その後、ボードを「北に向かって」いるように回転させます。
北のLED（LD3）が北を向いているはずです。

<!-- Now run the starter code and observe the output. What X, Y and Z values do you see? -->

ここで、スターターコードを実行し、その出力を観察します。X、Y、Zの値はどのようなっていますか？

``` console
$ # itmdump terminal
(..)
I16x3 { x: 45, y: 194, z: -3 }
I16x3 { x: 46, y: 195, z: -8 }
I16x3 { x: 47, y: 197, z: -2 }
```

<!-- 
Now rotate the board 90 degrees while keeping it parallel to the ground. What X, Y and Z values do
you see this time? Then rotate it 90 degrees again. What values do you see?
 -->

次に、地面と平行に保ちながら、ボードを90度回転します。今回は、X、Y、Zの値はどうなりましたか？
さらに90度回転します。値はどうなりましたか？
