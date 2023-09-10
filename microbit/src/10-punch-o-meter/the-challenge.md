# 挑战

为了简单起见，我们将在板保持水平的情况下仅测量X轴上的加速度。这样，我们就不必处理减去我们之前
观察到的*虚拟*的`1g`，这将是困难的，因为`1g`可能有X，Y，Z分量，这取决于电路板的方向。

冲压式流量计必须做到以下几点：

- 默认情况下，应用程序不会"观察"电路板的加速度。
- 当检测到显著的X加速度（即加速度超过某个阈值）时，应用程序应开始新的测量。
- 在测量间隔期间，应用程序应跟踪观察到的最大加速度。
- 测量间隔结束后，应用程序必须报告观察到的最大加速度。您可以使用`rprintln!`宏报告该值。

试一试，让我知道你打得有多用力 `;-)`.

> **注意**：有两个额外的API应该对我们尚未讨论的任务有用。 首先是[`set_accel_scale`]，您需要测量高g值。
> 其次是`embedded_hal`的[`Countdown`] trait。如果您决定使用它来保持测量间隔，则必须在[`nb::Result`]
> 类型上进行模式匹配，而不是使用我们在前几章中看到的`block!`宏。


[`set_accel_scale`]: https://docs.rs/lsm303agr/0.2.2/lsm303agr/struct.Lsm303agr.html#method.set_accel_scale
[`Countdown`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/timer/trait.CountDown.html
[`nb::Result`]: https://docs.rs/nb/1.0.0/nb/type.Result.html
