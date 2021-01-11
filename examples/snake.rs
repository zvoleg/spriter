#[macro_use]
extern crate spriter;
extern crate rand;

use spriter::{Key, Canvas, Color};

use std::time::Duration;
use std::collections::VecDeque;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Snake {
    body: VecDeque<(i32, i32)>,
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
    target: (i32, i32),
    canvas: Canvas,
    time_accumulator: Duration,
    step_time: Duration,
}

fn main() {
    let (handler, mut window) = spriter::init("Snake", 512, 512);
    let mut canvas = window.create_canvas(0, 0, 512, 512, 64, 64);
    canvas.set_clear_color(Color::new(0x22, 0x22, 0x55));
    let target = ((rand::random::<f32>() * 63.0).round() as i32, (rand::random::<f32>() * 63.0).round() as i32);
    let mut game = Game { snake: Snake::new(), target, canvas, time_accumulator: Duration::new(0, 0), step_time: Duration::from_secs_f32(1.0 / 10.0) };
    handler.run(window, move |frame_duration| {
        game.canvas.clear();
        if_pressed!(Key::W, {
            game.snake.direction = Direction::UP;
        });
        if_pressed!(Key::S, {
            game.snake.direction = Direction::DOWN;
        });
        if_pressed!(Key::A, {
            game.snake.direction = Direction::LEFT;
        });
        if_pressed!(Key::D, {
            game.snake.direction = Direction::RIGHT;
        });
        if_pressed!(Key::Escape, { spriter::program_stop() });

        game.time_accumulator = game.time_accumulator.checked_add(frame_duration).unwrap();
        if game.time_accumulator >= game.step_time {
            game.time_accumulator = Duration::new(0, 0);
            let first_part = game.snake.body.front().unwrap();
            let new_part = match game.snake.direction {
                Direction::UP => (first_part.0, first_part.1 - 1),
                Direction::DOWN => (first_part.0, first_part.1 + 1),
                Direction::LEFT => (first_part.0 - 1, first_part.1),
                Direction::RIGHT => (first_part.0 + 1, first_part.1),
            };
            game.snake.body.push_front(new_part);
            if new_part == game.target {
                game.target = ((rand::random::<f32>() * 63.0).round() as i32, (rand::random::<f32>() * 63.0).round() as i32);
            } else {
                game.snake.body.pop_back();
            }

            let mut color = Color::new(0xCC, 0x88, 0x88);
            for (x, y) in &game.snake.body {
                game.canvas.set_pixel(*x, *y, color).unwrap();
                color = Color::new(0x55, 0x55, 0xAA);
            }
            let target = &game.target;
            game.canvas.set_pixel(target.0, target.1, color).unwrap();
            true
        } else {
            false
        }
    });
}