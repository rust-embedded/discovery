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
