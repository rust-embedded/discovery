# LED roulette

Alright, let's start by building the following application:

<p align="center">
<img src="https://i.imgur.com/0k1r2Lc.gif">
</p>

I'm going to give you a high level API to implement this app but don't worry
we'll do low level stuff later on. The main goal of this chapter is to get
familiar with the "flashing" and debugging process.

Throughout this text we'll be using the starter code that's in the [discovery]
repository. Make sure you always have the latest version of the master branch
because this website tracks that branch.

The starter code is in the `src` directory of that repository. Inside that
directory there are more directories named after each chapter of this book. Most of
those directories are starter Cargo projects.

[discovery]: https://github.com/japaric/discovery

Now, jump into the `src/05-led-roulette` directory. Check the `src/main.rs`
file:

``` rust
#![no_std]
#![no_main]

extern crate pg;

#[no_mangle]
pub fn main() -> ! {
    let y;
    let x = 42;
    y = x;

    loop {}
}
```

There's some unusual stuff in it: `#![no_main]`, `#[no_mangle]` and `main` is
both `pub` and has signature `fn() -> !`. For now, why those are the way they
are doesn't matter. The only practical implication of all this is that you can't
return from the `main` function.

If you are a careful observer, you'll also notice there is a `.cargo` directory
in the Cargo project as well. `:-)`

Alright, let's start by building this program.
