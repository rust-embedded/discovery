# Async IO: The future

> **WARNING** Experimental code ahead!

So far, all the high level APIs we have been using have been of the blocking
variety. For example, the `delay::ms` function makes the processor wait for some
time to pass and during that wait the processor can't perform any useful action.
That's just wasteful. These blocking APIs make it hard (or near impossible) to
write programs that have to "do more than a thing".

The goal of this section will be to write a program that performs two concurrent
tasks: the "echo server" we wrote in section 10 and the LED roulette we wrote in
section 4.

To do that we'll have to throw away the high level APIs we have been using. The
"busy" waiting pattern (`while !condition {}`) we have been using must go as
well.

Instead we'll be using a new API based on "futures". We won't be used the
`futures` crate that's available in crates.io but a minimal version of the
trait:

``` rust
/// Trait for types which are a placeholder of a value that will become
/// available at possible some later point in time.
trait Future {
    type Item;

    /// Check if this future has completed
    fn poll(&mut self) -> Async<Self::Item>;

    /// Drive a future to its completion by continuously calling `poll`
    fn wait(mut self) -> Self::Item
        where Self: Sized
    {
        loop {
            if let Async::Ready(item) = self.poll() {
                return item;
            }
        }
    }
}

/// Return type of future, indicating whether a value is ready or not.
enum Async<T> {
    NotReady,
    Ready(T),
}
```
