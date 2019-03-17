<!-- # Busy waiting -->

# ビジーウェイト

<!-- 
The timer should now be properly initialized. All that's left is to implement the `delay` function
using the timer.
 -->

タイマは、適切に初期化されているはずです。残りは、このタイマを使って`delay`関数を実装することです。

<!-- 
First thing we have to do is set the autoreload register (`ARR`) to make the timer go off in `ms`
milliseconds. Because the counter operates at 1 KHz, the autoreload value will be the same as `ms`.
 -->

まず最初にやらなければならないことは、`ms`ミリ秒後にタイマをオフにするために、自動リロードレジスタ（`ARR`）レジスタを設定することです。
カウンタは1KHzで動作するため、自動リロードの値は、`ms`と同じ値になります。

``` rust
    // `ms`ティック後にオフになるようにタイマを設定します。
    // 1ティックは1msです。
    tim6.arr.write(|w| w.arr().bits(ms));
```

<!-- 
Next, we need to enable the counter. It will immediately start counting.
 -->

次に、カウンタを有効にする必要があります。有効化すると、すぐにカウントが開始します。

``` rust
    // CEN：カウンタを有効化します。
    tim6.cr1.modify(|_, w| w.cen().set_bit());
```

<!-- 
Now we need to wait until the counter reaches the value of the autoreload register, `ms`, then we'll
know that `ms` milliseconds have passed. That condition is known as an *update event* and its
indicated by the `UIF` bit of the status register (`SR`).
 -->

今度は、カウンタが自動リロードレジスタの値（`ms`）に到達するまで待つ必要があります。すると、`ms`ミリ秒経過したことがわかります。
この状態は、*更新イベント*と呼ばれます。そして、これはステータスレジスタ（`SR`）の`UIF`ビットによってわかります。

``` rust
    // アラームがオフになるまで（更新イベントが発生するまで）待ちます
    while !tim6.sr.read().uif().bit_is_set() {}
```

<!-- 
This pattern of just waiting until some condition is met, in this case that `UIF` becomes `1`, is
known as *busy waiting* and you'll see it a few more times in this text `:-)`.
 -->

ある条件が満たされるまで単純に待つようなパターン、今回の場合だと`UIF`が`1`になる、は*ビジーウェイト*と呼ばれます。
この言葉を、このテキスト内で何回か目にするでしょう`:-)`。

<!-- 
Finally, we must clear (set to `0`) this `UIF` bit. If we don't, next time we enter the `delay`
function we'll think the update event has already happened and skip over the busy waiting part.
 -->

最後に、`UIF`ビットをクリア（`0`に設定）しなければなりません。もしこれを行わないと、次に`delay`関数に入った時に、
更新イベントが既に発生しており、ビジーウェイト部分の実行を飛ばすことになります。

``` rust
    // 更新イベントフラグをクリアします
    tim6.sr.modify(|_, w| w.uif().clear_bit());
```

<!-- Now, put this all together and check if it works as expected. -->

では、ここまでの全てをまとめて、期待通り動くかどうか確認して下さい。