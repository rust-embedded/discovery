# LED roulette

Alright, let's start by building the following application:

<p align="center">
<img src="https://i.imgur.com/0k1r2Lc.gif">
</p>

I'm going to give you a high level API to implement this app but don't worry we'll do low level
stuff later on. The main goal of this chapter is to get familiar with the *flashing* and debugging
process.

Throughout this text we'll be using the starter code that's in the [discovery] repository. Make sure
you always have the latest version of the master branch because this website tracks that branch.

The starter code is in the `src` directory of that repository. Inside that directory there are more
directories named after each chapter of this book. Most of those directories are starter Cargo projects.

[discovery]: https://github.com/japaric/discovery

Now, jump into the `src/05-led-roulette` directory. Check the `src/main.rs`
file:

``` rust
#![deny(unsafe_code)]
#![no_std]

extern crate aux5;

fn main() {
    let _y;
    let x = 42;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
```

The only thing that should look different from a standard Rust program is the addition of the
`#![no_std]` attribute. That attribute says that this program won't use the `std` crate, which
assumes an underlying OS, but the `core` crate, a subset of `std` that can run on bare metal systems
(i.e., systems without an OS).

If you are a careful observer, you'll also notice there is a `.cargo` directory in the Cargo project
as well. `:-)`

Alright, let's start by building this program.
