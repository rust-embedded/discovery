use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use microbit::display::nonblocking::Display;
use microbit::gpio::DisplayPins;
use microbit::pac;
use microbit::pac::{interrupt, TIMER1};
use tiny_led_matrix::Render;

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

#[interrupt]
fn TIMER1() {
    free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    })
}