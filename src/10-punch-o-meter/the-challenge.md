# The challenge

To keep things simple, we'll measure the acceleration only in the X axis while the board remains
horizontal. That way we won't have to deal with subtracting that *fictitious* `1g` we observed
before which would be hard because that `1g` could have X Y Z components depending on how the board
is oriented.

Here's what the punch-o-meter must do:

- By default, the app is not "observing" the acceleration of the board.
- When a significant X acceleration is detected (i.e. the acceleration goes above some threshold),
  the app should start a new measurement.
- During that measurement interval, the app should keep track of the maximum acceleration observed
- After the measurement interval ends, the app must report the maximum acceleration observed. You
  can report the value using the `rprintln!` macro.

Give it a try and let me know how hard you can punch `;-)`.

> **NOTE** There are two additional APIs that should be useful for this task we haven't discussed yet.
> First the [`set_accel_scale`] one which you need to measure high g values.
> Secondly the [`Countdown`] trait from `embedded_hal`. If you decide to use this to keep your measurement
> intervals you will have to pattern match on the [`nb::Result`] type instead of using the `block!` macro
  we have seen in previous chapters.


[`set_accel_scale`]: https://docs.rs/lsm303agr/0.2.2/lsm303agr/struct.Lsm303agr.html#method.set_accel_scale
[`Countdown`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/timer/trait.CountDown.html
[`nb::Result`]: https://docs.rs/nb/1.0.0/nb/type.Result.html
