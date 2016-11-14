# The challenge

To keep things simple, we'll measure the acceleration only in the X axis while
the board remains horizontal. That way we won't have to deal with subtracting
that "fictitious" `1 g` we observed before which would be hard because that `1
g` could have X Y Z components depending on how the board is oriented.

Here's what the punch-o-meter must do:

- By default, the app is not "observing" the acceleration of the board.
- When a significant X acceleration is detected (i.e. the acceleration goes
  above some threshold), the app should start a new measurement.
- During that measurement interval, the app should keep track of the maximum
  acceleration observed
- After the measurement interval ends, the app must report the maximum
  acceleration observed. You can report the value using the `iprintln` macro.

Give it a try and let me know how hard you can punch `;-)`.
