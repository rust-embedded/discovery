<!-- # The challenge -->

# 課題

<!-- 
To keep things simple, we'll measure the acceleration only in the X axis while the board remains
horizontal. That way we won't have to deal with subtracting that *fictitious* `1g` we observed
before which would be hard because that `1g` could have X Y Z components depending on how the board
is oriented.
 -->

簡単化のために、ボードを水平にしたままX軸の加速度だけを計測します。
この方法では、前回観測した*架空の*`1g`を差し引く必要がなくなります。架空の1gを扱うことは、難しいです。
なぜなら、`1g`はボードが向いている方向によって、X Y Zの要素を持つからです。

<!-- Here's what the punch-o-meter must do: -->

パンチングマシンがやらなければいけないことは、次の通りです。

<!-- 
- By default, the app is not "observing" the acceleration of the board.
- When a significant X acceleration is detected (i.e. the acceleration goes above some threshold),
  the app should start a new measurement.
- During that measurement interval, the app should keep track of the maximum acceleration observed
- After the measurement interval ends, the app must report the maximum acceleration observed. You
  can report the value using the `iprintln` macro.
 -->

- デフォルトでは、ボードは加速度を「観測」していません。
- 大きなX軸方向の加速度が検出された（つまり、加速度がしきい値を超えた）時、アプリケーションは新しい計測を開始します。
- 計測期間中、観測した最大の加速度を追跡し続ける必要があります。
- 観測期間を終えると、アプリケーションは、観測した最大加速度を報告します。`iprintln`マクロを使って最大加速度を報告できます。

<!-- Give it a try and let me know how hard you can punch `;-)`. -->

パンチングマシンを試してみて、あなたのパンチがどのくらい強力か、私に教えて下さい `;-)`。
