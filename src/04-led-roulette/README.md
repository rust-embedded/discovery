# LED roulette

Alright, let's start by building the following application:

<p align="center">
<img src="https://i.imgur.com/0k1r2Lc.gif">
</p>

I'm going to give you a high level API to implement this app but don't worry
we'll do low level stuff later on. The main goal of this chapter is to get
familiar with the "flashing" and debugging process.

Jump into the `src/04-led-roulette` directory. There's some starter code in it.

``` rust
#![no_std]
#![no_main]

extern crate pg;

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    let y;
    let x = 42;
    y = x;

    loop {}
}
```

There's some unusual stuff in it: `#![no_main]`, `#[export_name]` and `main` is
both `pub` and has signature `fn() -> !`. For now, why those are the way they
are doesn't matter. The only practical implication of all this is that you can't
return from the `main` function.

If you are a careful observer, you'll also notice there is a `.cargo` directory
in the Cargo project as well. `:-)`

Alright, let's start by building this program.
