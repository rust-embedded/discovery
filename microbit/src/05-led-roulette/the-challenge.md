# The challenge

You are now well armed to face a challenge! Your task will be to implement the application I showed
you at the beginning of this chapter.

<p align="center">
<video src="../assets/roulette_fast.mp4" loop autoplay>
</p>

If you can't exactly see what's happening here it is in a much slower version:

<p align="center">
<video src="../assets/roulette_slow.mp4" loop autoplay>
</p>

Since working with the LED pins separately is quite annoying
(especially if you have to use basically all of them like here)
you can use the display API provided by the BSP. It works like this:

```rust
{{#include examples/the-challenge.rs}}
```

Equipped with this API your task basically boils down to just having
to calculate the proper image matrix and passing it into the BSP.
