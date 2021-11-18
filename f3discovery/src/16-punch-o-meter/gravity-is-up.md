# Gravity is up?

What's the first thing we'll do?

Perform a sanity check!

The starter code prints the X, Y and Z components of the acceleration measured by the accelerometer.
The values have already been "scaled" and have units of `g`s. Where `1 g` is equal to the
acceleration of the gravity, about `9.8` meters per second squared.

``` rust
{{#include src/main.rs}}
```

The output of this program with the board sitting still will be something like:

``` console
$ # itmdump console
(..)
(0.0, 0.0, 1.078125)
(0.0, 0.0, 1.078125)
(0.0, 0.0, 1.171875)
(0.0, 0.0, 1.03125)
(0.0, 0.0, 1.078125)
```

Which is weird because the board is not moving yet its acceleration is non-zero. What's going on?
This must be related to the gravity, right? Because the acceleration of gravity is `1 g`. But the
gravity pulls objects downwards so the acceleration along the Z axis should be negative not positive
...

Did the program get the Z axis backwards? Nope, you can test rotating the board to align the gravity
to the X or Y axis but the acceleration measured by the accelerometer is always pointing up.

What happens here is that the accelerometer is measuring the *proper acceleration* of the board not
the acceleration *you* are observing. This proper acceleration is the acceleration of the board as
seen from a observer that's in free fall. An observer that's in free fall is moving toward the
center of the the Earth with an acceleration of `1g`; from its point of view the board is actually
moving upwards (away from the center of the Earth) with an acceleration of `1g`. And that's why the
proper acceleration is pointing up. This also means that if the board was in free fall, the
accelerometer would report a proper acceleration of zero. Please, don't try that at home.

Yes, physics is hard. Let's move on.
