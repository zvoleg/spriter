#[macro_use]
extern crate spriter;
extern crate rand;

use spriter::{Program, Key, Canvas};

use std::time::Duration;
use std::collections::VecDeque;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Snake {
    body: VecDeque<(u32, u32)>,
    direction: Direction,
}

impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::with_capacity(3);
        body.push_back((32, 32));
        body.push_back((31, 32));
        body.push_back((30, 32));
        let direction = Direction::RIGHT;
        Self { body, direction }
    }
}

struct Game {
    snake: Snake,
    target: (u32, u32),
    canvas: Canvas,
    time_accumulator: Duration,
    step_time: Duration,
    run: bool,
}

impl Program for Game {
    fn execute(&mut self, frame_duration: Duration) -> bool {
        self.canvas.clear();
        if_pressed!(Key::W, {
            self.snake.direction = Direction::UP;
        });
        if_pressed!(Key::S, {
            self.snake.direction = Direction::DOWN;
        });
        if_pressed!(Key::A, {
            self.snake.direction = Direction::LEFT;
        });
        if_pressed!(Key::D, {
            self.snake.direction = Direction::RIGHT;
        });
        if_pressed!(Key::Escape, {self.run = false});

        self.time_accumulator = self.time_accumulator.checked_add(frame_duration).unwrap();
        if self.time_accumulator >= self.step_time {
            self.time_accumulator = Duration::new(0, 0);
            let first_part = self.snake.body.front().unwrap();
            let new_part = match self.snake.direction {
                Direction::UP => (first_part.0, first_part.1 - 1),
                Direction::DOWN => (first_part.0, first_part.1 + 1),
                Direction::LEFT => (first_part.0 - 1, first_part.1),
                Direction::RIGHT => (first_part.0 + 1, first_part.1),
            };
            self.snake.body.push_front(new_part);
            if new_part == self.target {
                self.target = ((rand::random::<f32>() * 63.0).round() as u32, (rand::random::<f32>() * 63.0).round() as u32);
            } else {
                self.snake.body.pop_back();
            }

            let mut color = 0xCC8888;
            for (x, y) in &self.snake.body {
                self.canvas.set_pixel(*x, *y, color);
                color = 0x5555AA;
            }
            let target = &self.target;
            self.canvas.set_pixel(target.0, target.1, 0x33FF33);
            true
        } else {
            false
        }
    }

    fn is_run(&self) -> bool {
        self.run
    }
}

fn main() {
    let (handler, mut window) = spriter::init("Snake", 512, 512);
    let mut canvas = window.create_canvas(0, 0, 512, 512, 64, 64);
    canvas.set_clear_color(0x222255);
    let target = ((rand::random::<f32>() * 63.0).round() as u32, (rand::random::<f32>() * 63.0).round() as u32);
    let game = Game { snake: Snake::new(), target, canvas, time_accumulator: Duration::new(0, 0), step_time: Duration::from_secs_f32(1.0 / 10.0), run: true };
    handler.run(game, window);
}