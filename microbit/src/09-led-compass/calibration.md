# Calibration

One very important thing to do before using a sensor and trying to develop
an application using it is verifying that it's output is actually correct.
If this does not happen to be the case we need to calibrate the sensor
(alternatively it could also be broken but that's rather unlikely in this case).

In my case on two different micro:bit's the magnetometer, without calibration,
was quite a bit off of what it is supposed to measure. Hence for the purposes
of this chapter we will just assume that the sensor has to be calibrated.

The calibration involves quite a bit of math (matrices) so we won't cover it here but this
[Design Note] describes the procedure if you are interested. 

[Design Note]: https://www.st.com/resource/en/design_tip/dt0103-compensating-for-magnetometer-installation-error-and-hardiron-effects-using-accelerometerassisted-2d-calibration-stmicroelectronics.pdf

Luckily for us though the group that built the original software for the
micro:bit already implemented a calibration mechanism in C++ over [here].

[here]: https://github.com/lancaster-university/codal-microbit-v2/blob/006abf5566774fbcf674c0c7df27e8a9d20013de/source/MicroBitCompassCalibrator.cpp

You can find a translation of it to Rust in `src/calibration.rs`. The usage
is demonstrated in the default `src/main.rs` file. The way the calibration
works is illustrated in this video:

<p align="center">
<video src="https://video.microbit.org/support/compass+calibration.mp4" loop autoplay>
</p>

You have to basically tilt the micro:bit until all the LEDs on the LED matrix light up.

If you do not want to play the game every time you restart your application during development
feel free to modify the `src/main.rs` template to just use the same static calibration
once you got the first one.

Now where we got the sensor calibration out of the way let's look into
actually building this application!
