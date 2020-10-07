# The challenge

You are now well armed to face a challenge! Your task will be to implement the application I showed
you at the beginning of this chapter.

<p align="center">
<img src="https://imgur.com/download/RWBWYX4/">
</p>

If you can't exactly see what's happening here it is in a much slower version:

<p align="center">
<img src="https://imgur.com/download/o9MyyCF/">
</p>

2 hints before you start:

1. As we learned before the LED matrix of the micro:bit is actually a 3x9 while being exposed as a 5x5. Furthermore
   it seems like the 9 columns and 3 rows are more or less randomly mapped to the visual 5x5 matrix. If you don't want
   to go through the effort of figuring out the pins you have to set high/low in order to blink the border of the
   matrix, here is the list: `(R1, C1) (R2, C4) (R1, C2), (R2, C5) (R1, C3) (R3, C8) (R2, C1) (R1, C4) (R3, C2) (R2,
   C6) (R3, C1) (R2, C7) (R3, C3) (R1, C8) (R2, C2) (R3, C4)`

2. If you are thinking about storing columns and rows in arrays you will quickly notice they are of different type since
   all GPIO pins are represented as their own type. However you can call `.degrade()` on the individual GPIO objects in
   order to "degrade" them all into the same type and then store them in an array.
