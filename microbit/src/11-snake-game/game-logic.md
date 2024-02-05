# Game logic

First, we are going to describe the game logic. You are probably familiar with snake games, but if not, the basic idea
is that the player guides a snake around a 2D grid. At any given time, there is some "food" at a random location on the
grid and the goal of the game is to get the snake to "eat" as much food as possible. Each time the snake eats some food
it grows in length. The player loses if the snake crashes into its own tail. In some variants of the game, the player
also loses if the snake crashes into the edge of the grid, but given the small size of our grid we are going to
implement a "wraparound" rule where, if the snake goes off one edge of the grid, it will continue from the opposite
edge.

## The `game` module

The code in this section should go in a separate file, `game.rs`, in our `src` directory.

```rust
use heapless::FnvIndexSet;

/// A single point on the grid.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coords {
   // Signed ints to allow negative values (handy when checking if we have gone
   // off the top or left of the grid)
   row: i8,
   col: i8
}

impl Coords {
   /// Get random coordinates within a grid. `exclude` is an optional set of
   /// coordinates which should be excluded from the output.
   fn random(
      rng: &mut Prng,  // We define the Prng struct below
      exclude: Option<&FnvIndexSet<Coords, 32>>
   ) -> Self {
      let mut coords = Coords {
         row: ((rng.random_u32() as usize) % 5) as i8,
         col: ((rng.random_u32() as usize) % 5) as i8
      };
      while exclude.is_some_and(|exc| exc.contains(&coords)) {
         coords = Coords {
            row: ((rng.random_u32() as usize) % 5) as i8,
            col: ((rng.random_u32() as usize) % 5) as i8
         }
      }
      coords
   }

   /// Whether the point is outside the bounds of the grid.
   fn is_out_of_bounds(&self) -> bool {
      self.row < 0 || self.row >= 5 || self.col < 0 || self.col >= 5
   }
}
```

We use a `Coords` struct to refer to a position on the grid. Because `Coords` only contains two integers, we tell the
compiler to derive an implementation of the `Copy` trait for it, so we can pass around `Coords` structs without having
to worry about ownership.

We define an associated function, `Coords::random`, which will give us a random position on the grid. We will use this
later to determine where to place the snake's food. To do this, we need a source of random numbers. The nRF52833 has a
random number generator (RNG) peripheral, documented at section 6.19 of the [spec sheet](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.3.pdf). The HAL gives us a simple
interface to the RNG via the `microbit::hal::rng::Rng` struct. However, it is a blocking interface, and the time
needed to generate one random byte of data is variable and unpredictable. We therefore define a [pseudo-random](https://en.wikipedia.org/wiki/Pseudorandom_number_generator)
number generator (PRNG) which uses an [xorshift](https://en.wikipedia.org/wiki/Xorshift) algorithm to generate
pseudo-random `u32` values that we can use to determine where to place food. The algorithm is basic and not
cryptographically secure, but it is efficient, easy to implement and good enough for our humble snake game. Our `Prng`
struct requires an initial seed value, which we get from the RNG peripheral.

```rust
/// A basic pseudo-random number generator.
struct Prng {
    value: u32
}

impl Prng {
    fn new(seed: u32) -> Self {
        Self {value: seed}
    }

    /// Basic xorshift PRNG function: see https://en.wikipedia.org/wiki/Xorshift
    fn xorshift32(mut input: u32) -> u32 {
        input ^= input << 13;
        input ^= input >> 17;
        input ^= input << 5;
        input
    }

    /// Return a pseudo-random u32.
    fn random_u32(&mut self) -> u32 {
        self.value = Self::xorshift32(self.value);
        self.value
    }
}
```

We also need to define a few `enum`s that help us manage the game's state: direction of movement, direction to turn, the
current game status and the outcome of a particular "step" in the game (ie, a single movement of the snake).

```rust
/// Define the directions the snake can move.
enum Direction {
    Up,
    Down,
    Left,
    Right
}

/// What direction the snake should turn.
#[derive(Debug, Copy, Clone)]
pub enum Turn {
    Left,
    Right,
    None
}

/// The current status of the game.
pub enum GameStatus {
    Won,
    Lost,
    Ongoing
}

/// The outcome of a single move/step.
enum StepOutcome {
    /// Grid full (player wins)
    Full(Coords),
    /// Snake has collided with itself (player loses)
    Collision(Coords),
    /// Snake has eaten some food
    Eat(Coords),
    /// Snake has moved (and nothing else has happened)
    Move(Coords)
}
```

Next up we define a `Snake` struct, which keeps track of the coordinates occupied by the snake and its direction of
travel. We use a queue (`heapless::spsc::Queue`) to keep track of the order of coordinates and a hash set
(`heapless::FnvIndexSet`) to allow for quick collision detection.  The `Snake` has methods to allow it to move.

```rust
use heapless::spsc::Queue;

// ...

struct Snake {
    /// Coordinates of the snake's head.
    head: Coords,
    /// Queue of coordinates of the rest of the snake's body. The end of the tail is
    /// at the front.
    tail: Queue<Coords, 32>,
    /// A set containing all coordinates currently occupied by the snake (for fast
    /// collision checking).
    coord_set: FnvIndexSet<Coords, 32>,
    /// The direction the snake is currently moving in.
    direction: Direction
}

impl Snake {
    fn new() -> Self {
        let head = Coords { row: 2, col: 2 };
        let initial_tail = Coords { row: 2, col: 1 };
        let mut tail = Queue::new();
        tail.enqueue(initial_tail).unwrap();
        let mut coord_set: FnvIndexSet<Coords, 32> = FnvIndexSet::new();
        coord_set.insert(head).unwrap();
        coord_set.insert(initial_tail).unwrap();
        Self {
            head,
            tail,
            coord_set,
            direction: Direction::Right,
        }
    }

    /// Move the snake onto the tile at the given coordinates. If `extend` is false,
    /// the snake's tail vacates the rearmost tile.
    fn move_snake(&mut self, coords: Coords, extend: bool) {
        // Location of head becomes front of tail
        self.tail.enqueue(self.head).unwrap();
        // Head moves to new coords
        self.head = coords;
        self.coord_set.insert(coords).unwrap();
        if !extend {
            let back = self.tail.dequeue().unwrap();
            self.coord_set.remove(&back);
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }
    }

    fn turn(&mut self, direction: Turn) {
        match direction {
            Turn::Left => self.turn_left(),
            Turn::Right => self.turn_right(),
            Turn::None => ()
        }
    }
}
```

The `Game` struct keeps track of the game state. It holds a `Snake` object, the current coordinates of the food, the
speed of the game (which is used to determine the time that elapses between each movement of the snake), the status of
the game (whether the game is ongoing or the player has won or lost) and the player's score.

This struct contains methods to handle each step of the game, determining the snake's next move and updating the game
state accordingly. It also contains two methods--`game_matrix` and `score_matrix`--that output 2D arrays of values
which can be used to display the game state or the player score on the LED matrix (as we will see later).

```rust
/// Struct to hold game state and associated behaviour
pub(crate) struct Game {
    rng: Prng,
    snake: Snake,
    food_coords: Coords,
    speed: u8,
    pub(crate) status: GameStatus,
    score: u8
}

impl Game {
    pub(crate) fn new(rng_seed: u32) -> Self {
        let mut rng = Prng::new(rng_seed);
        let mut tail: FnvIndexSet<Coords, 32> = FnvIndexSet::new();
        tail.insert(Coords { row: 2, col: 1 }).unwrap();
        let snake = Snake::new();
        let food_coords = Coords::random(&mut rng, Some(&snake.coord_set));
        Self {
            rng,
            snake,
            food_coords,
            speed: 1,
            status: GameStatus::Ongoing,
            score: 0
        }
    }

    /// Reset the game state to start a new game.
    pub(crate) fn reset(&mut self) {
        self.snake = Snake::new();
        self.place_food();
        self.speed = 1;
        self.status = GameStatus::Ongoing;
        self.score = 0;
    }

    /// Randomly place food on the grid.
    fn place_food(&mut self) -> Coords {
        let coords = Coords::random(&mut self.rng, Some(&self.snake.coord_set));
        self.food_coords = coords;
        coords
    }

    /// "Wrap around" out of bounds coordinates (eg, coordinates that are off to the
    /// left of the grid will appear in the rightmost column). Assumes that
    /// coordinates are out of bounds in one dimension only.
    fn wraparound(&self, coords: Coords) -> Coords {
        if coords.row < 0 {
            Coords { row: 4, ..coords }
        } else if coords.row >= 5 {
            Coords { row: 0, ..coords }
        } else if coords.col < 0 {
            Coords { col: 4, ..coords }
        } else {
            Coords { col: 0, ..coords }
        }
    }

    /// Determine the next tile that the snake will move on to (without actually
    /// moving the snake).
    fn get_next_move(&self) -> Coords {
        let head = &self.snake.head;
        let next_move = match self.snake.direction {
            Direction::Up => Coords { row: head.row - 1, col: head.col },
            Direction::Down => Coords { row: head.row + 1, col: head.col },
            Direction::Left => Coords { row: head.row, col: head.col - 1 },
            Direction::Right => Coords { row: head.row, col: head.col + 1 },
        };
        if next_move.is_out_of_bounds() {
            self.wraparound(next_move)
        } else {
            next_move
        }
    }

    /// Assess the snake's next move and return the outcome. Doesn't actually update
    /// the game state.
    fn get_step_outcome(&self) -> StepOutcome {
        let next_move = self.get_next_move();
        if self.snake.coord_set.contains(&next_move) {
            // We haven't moved the snake yet, so if the next move is at the end of
            // the tail, there won't actually be any collision (as the tail will have
            // moved by the time the head moves onto the tile)
            if next_move != *self.snake.tail.peek().unwrap() {
                StepOutcome::Collision(next_move)
            } else {
                StepOutcome::Move(next_move)
            }
        } else if next_move == self.food_coords {
            if self.snake.tail.len() == 23 {
                StepOutcome::Full(next_move)
            } else {
                StepOutcome::Eat(next_move)
            }
        } else {
            StepOutcome::Move(next_move)
        }
    }

    /// Handle the outcome of a step, updating the game's internal state.
    fn handle_step_outcome(&mut self, outcome: StepOutcome) {
        self.status = match outcome {
            StepOutcome::Collision(_) => GameStatus::Lost,
            StepOutcome::Full(_) => GameStatus::Won,
            StepOutcome::Eat(c) => {
                self.snake.move_snake(c, true);
                self.place_food();
                self.score += 1;
                if self.score % 5 == 0 {
                    self.speed += 1
                }
                GameStatus::Ongoing
            },
            StepOutcome::Move(c) => {
                self.snake.move_snake(c, false);
                GameStatus::Ongoing
            }
        }
    }

    pub(crate) fn step(&mut self, turn: Turn) {
        self.snake.turn(turn);
        let outcome = self.get_step_outcome();
        self.handle_step_outcome(outcome);
    }

    /// Calculate the length of time to wait between game steps, in milliseconds.
    /// Generally this will get lower as the player's score increases, but need to
    /// be careful it cannot result in a value below zero.
    pub(crate) fn step_len_ms(&self) -> u32 {
        let result = 1000 - (200 * ((self.speed as i32) - 1));
        if result < 200 {
            200u32
        } else {
            result as u32
        }
    }

    /// Return an array representing the game state, which can be used to display the
    /// state on the microbit's LED matrix. Each `_brightness` parameter should be a
    /// value between 0 and 9.
    pub(crate) fn game_matrix(
        &self,
        head_brightness: u8,
        tail_brightness: u8,
        food_brightness: u8
    ) -> [[u8; 5]; 5] {
        let mut values = [[0u8; 5]; 5];
        values[self.snake.head.row as usize][self.snake.head.col as usize] = head_brightness;
        for t in &self.snake.tail {
            values[t.row as usize][t.col as usize] = tail_brightness
        }
        values[self.food_coords.row as usize][self.food_coords.col as usize] = food_brightness;
        values
    }

    /// Return an array representing the game score, which can be used to display the
    /// score on the microbit's LED matrix (by illuminating the equivalent number of
    /// LEDs, going left->right and top->bottom).
    pub(crate) fn score_matrix(&self) -> [[u8; 5]; 5] {
        let mut values = [[0u8; 5]; 5];
        let full_rows = (self.score as usize) / 5;
        for r in 0..full_rows {
            values[r] = [1; 5];
        }
        for c in 0..(self.score as usize) % 5 {
            values[full_rows][c] = 1;
        }
        values
    }
}
```

## The `main` file

The following code should be placed in our `main.rs` file. 

```rust
#![no_main]
#![no_std]

mod game;

use cortex_m_rt::entry;
use microbit::{
   Board,
   hal::{prelude::*, Rng, Timer},
   display::blocking::Display
};
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use crate::game::{Game, GameStatus, Turn};

#[entry]
fn main() -> ! {
   rtt_init_print!();
   let mut board = Board::take().unwrap();
   let mut timer = Timer::new(board.TIMER0);
   let mut rng = Rng::new(board.RNG);
   let mut game = Game::new(rng.random_u32());
   let mut display = Display::new(board.display_pins);

   loop {
      loop {  // Game loop
         let image = game.game_matrix(9, 9, 9);
         // The brightness values are meaningless at the moment as we haven't yet
         // implemented a display capable of displaying different brightnesses
         display.show(&mut timer, image, game.step_len_ms());
         match game.status {
            GameStatus::Ongoing => game.step(Turn::None), // Placeholder as we
                                                          // haven't implemented
                                                          // controls yet
            _ => {
               for _ in 0..3 {
                  display.clear();
                  timer.delay_ms(200u32);
                  display.show(&mut timer, image, 200);
               }
               display.clear();
               display.show(&mut timer, game.score_matrix(), 1000);
               break
            }
         }
      }
      game.reset();
   }
}
```

After initialising the board and its timer and RNG peripherals, we initialise a `Game` struct and a `Display` from the
`microbit::display::blocking` module.

In our "game loop" (which runs inside of the "main loop" we place in our `main` function), we repeatedly perform the
following steps:

1. Get a 5x5 array of bytes representing the grid. The `Game::get_matrix` method takes three integer arguments (which 
   should be between 0 and 9, inclusive) which will, eventually, represent how brightly the head, tail and food should be
   displayed. The basic `Display` we are using at this point does not support variable brightness, so we just provide 
   values of 9 for each (but any non-zero value would work) at this stage.
2. Display the matrix, for an amount of time determined by the `Game::step_len_ms` method. As currently implemented,
   this method basically provides for 1 second between steps, reducing by 200ms every time the player scores 5 points 
   (eating 1 piece of food = 1 point), subject to a floor of 200ms.
3. Check the game status. If it is `Ongoing` (which is its initial value), run a step of the game and update the game
   state (including its `status` property). Otherwise, the game is over, so flash the current image three times, then
   show the player's score (represented as a number of illuminated LEDs corresponding to the score), and exit the game
   loop.

Our main loop just runs the game loop repeatedly, resetting the game's state after each iteration.

If you run this, you should see two LEDs illuminated halfway down the display (the snake's head in the middle and its
tail to the left). You will also see another LED illuminated somewhere on the board, representing the snake's food.
Approximately each second, the snake will move one space to the right.

Next we will add an ability to control the snake's movements.