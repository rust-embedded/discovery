# Using the non-blocking display

We now have a basic functioning snake game. But you might find that when the snake gets a bit longer, it can be 
difficult to tell the snake from the food, and to tell which direction the snake is heading, because all LEDs are the
same brightness. Let's fix that.

The `microbit` library makes available two different interfaces to the LED matrix: a basic, blocking interface, which
we have been using, and a non-blocking interface which allows you to customise the brightness of each LED. At the
hardware level, each LED is either "on" or "off", but the `microbit::display::nonblocking` module simulates ten levels
of brightness for each LED by rapidly switching the LED on and off.

The code to interact with the non-blocking interface is pretty simple and will follow a similar structure to the code we
used to interact with the buttons.

```rust
use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use microbit::display::nonblocking::Display;
use microbit::gpio::DisplayPins;
use microbit::pac;
use microbit::pac::TIMER1;

static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));

pub(crate) fn init_display(board_timer: TIMER1, board_display: DisplayPins) {
    let display = Display::new(board_timer, board_display);

    free(move |cs| {
        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
    });
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIMER1)
    }
}
```

First, we initialise a `microbit::display::nonblocking::Display` struct representing the LED display, passing it the
board's `TIMER1` and `DisplayPins` peripherals. Then we store the display in a Mutex. Finally, we unmask the `TIMER1`
interrupt.

We then define a couple of convenience functions which allow us to easily set (or unset) the image to be displayed.

```rust
use tiny_led_matrix::Render;

// ...

/// Display an image.
pub(crate) fn display_image(image: &impl Render) {
    free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.show(image);
        }
    })
}

/// Clear the display (turn off all LEDs).
pub(crate) fn clear_display() {
    free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.clear();
        }
    })
}
```

`display_image` takes an image and tells the display to show it. Like the `Display::show` method that it calls, this
function takes a struct that implements the `tiny_led_matrix::Render` trait. That trait ensures that the struct contains
the data and methods necessary for the `Display` to render it on the LED matrix. The two implementations of `Render`
provided by the `microbit::display::nonblocking` module are `BitImage` and `GreyscaleImage`. In a `BitImage`, each
"pixel" (or LED) is either illuminated or not (like when we used the blocking interface), whereas in a
`GreyscaleImage` each "pixel" can have a different brightness.

`clear_display` does exactly as the name suggests.

Finally, we use the `interrupt` macro to define a handler for the `TIMER1` interrupt. This interrupt fires many times a
second, and this is what allows the `Display` to rapidly cycle the different LEDs on and off to give the illusion of
varying brightness levels. All our handler code does is call the `Display::handle_display_event` method, which handles
this.

```rust
use microbit::pac::interrupt;

// ...

#[interrupt]
fn TIMER1() {
    free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    })
}
```

Now we just need to update our `main` function to call `init_display` and use the new functions we have defined to
interact with our fancy new display.

```rust
#![no_main]
#![no_std]

mod game;
mod control;
mod display;

use cortex_m_rt::entry;
use microbit::{
    Board,
    hal::{prelude::*, Rng, Timer},
    display::nonblocking::{BitImage, GreyscaleImage}
};
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

use crate::control::{get_turn, init_buttons};
use crate::display::{clear_display, display_image, init_display};
use crate::game::{Game, GameStatus};


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0).into_periodic();
    let mut rng = Rng::new(board.RNG);
    let mut game = Game::new(rng.random_u32());

    init_buttons(board.GPIOTE, board.buttons);
    init_display(board.TIMER1, board.display_pins);


    loop {
        loop {  // Game loop
            let image = GreyscaleImage::new(&game.game_matrix(6, 3, 9));
            display_image(&image);
            timer.delay_ms(game.step_len_ms());
            match game.status {
                GameStatus::Ongoing => game.step(get_turn(true)),
                _ => {
                    for _ in 0..3 {
                        clear_display();
                        timer.delay_ms(200u32);
                        display_image(&image);
                        timer.delay_ms(200u32);
                    }
                    clear_display();
                    display_image(&BitImage::new(&game.score_matrix()));
                    timer.delay_ms(2000u32);
                    break
                }
            }
        }
        game.reset();
    }
}
```