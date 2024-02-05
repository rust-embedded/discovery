# Snake game

We're now going to implement a basic [snake](https://en.wikipedia.org/wiki/Snake_(video_game_genre)) game that you can play on a micro:bit v2 using its 5x5 LED matrix as a
display and its two buttons as controls. In doing so, we will build on some of the concepts covered in the earlier
chapters of this book, and also learn about some new peripherals and concepts.

In particular, we will be using the concept of hardware interrupts to allow our program to interact with multiple
peripherals at once. Interrupts are a common way to implement concurrency in embedded contexts. There is a good
introduction to concurrency in an embedded context in the [Embedded Rust Book](https://docs.rust-embedded.org/book/concurrency/index.html) that I suggest you read through
before proceeding.

> **NOTE** This chapter has been developed for the micro:bit v2 only, not the v1. Contributions to port the code to the
> v1 are welcome.

> **NOTE** In this chapter, we are going to use later versions of certain libraries that have been used in previous
> chapters. We are going to use version 0.13.0 of the `microbit` library (the preceding chapters have used 0.12.0).
> Version 0.13.0 fixes a couple of bugs in the non-blocking display code that we will be using. We are also going to use
> version 0.8.0 of the `heapless` library (previous chapters used version 0.7.10), which allows us to use certain of its
> data structures with structs that implement Rust's `core::Hash` trait.