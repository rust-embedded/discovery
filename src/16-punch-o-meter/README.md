<!-- # Punch-o-meter -->

# パンチングマシン

<!-- In this section we'll be playing with the accelerometer that's in the board. -->

このセクションでは、ボードの加速度計で遊びます。

<!-- 
What are we building this time? A punch-o-meter! We'll be measuring the power of your jabs. Well,
actually the maximum acceleration that you can reach because acceleration is what accelerometers
measure. Strength and acceleration are proportional though so it's a good approximation.
 -->

今回は何を作るのでしょうか？パンチングマシンです！あなたのジャブの強さを計測します。
実際は、到達した最高の加速度を計測します。なぜなら、加速度計が計測するのは加速度だからです。
強さと加速度は比例するため、これは良い近似です。

<!-- 
The accelerometer is also built inside the LSM303DLHC package. And just like the magnetometer, it
can also be accessed using the I2C bus. It also has the same coordinate system as the magnetometer.
Here's the coordinate system again:
 -->

加速度計もLSM303DLHCパッケージに組み込まれています。磁力計と同様に、I2Cバスを使ってアクセスできます。
加速度計も磁力計と同じ座標系システムを持っています。座標系を再び示します。

<p align="center">
<img height=480 title="Magnetometer axes" src="../assets/f3-lsm303dlhc.png">
</p>

<!-- 
Just like in the previous unit, we'll be using a high level API to directly get the sensor readings
in a nicely packaged `struct`.
 -->

前回と同様、高レベルのAPIを使用します。センサから読み取った値を、`struct`で良い感じにパッケージして直接取得します。
