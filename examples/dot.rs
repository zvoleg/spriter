#[macro_use]
extern crate spriter;

use spriter::{Canvas, Color, Program, Key};

use std::time::Duration;

struct Dot {
    x: f32,
    y: f32,
    color: u32,
    canvas: Canvas,
    run: bool,
    time_accumulator: Duration,
}

impl Program for Dot {
    fn execute(&mut self, frame_duration: Duration) -> bool {
        self.canvas.clear();
        let new_time = self.time_accumulator.checked_add(frame_duration).unwrap();
        self.time_accumulator = new_time;
        if_pressed!(Key::W, {self.y -= 0.001;});
        if_pressed!(Key::S, {self.y += 0.001;});
        if_pressed!(Key::A, {self.x -= 0.001;});
        if_pressed!(Key::D, {self.x += 0.001;});
        handle_press!(Key::Space, {
            self.color += 0x131313;
            self.color &= 0xFFFFFF;
        });
        if_pressed!(Key::Escape, {self.run = false;});
        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.x > 127.0 {
            self.x = 127.0;
        }
        if self.y < 0.0 {
            self.y = 0.0;
        }
        if self.y > 127.0 {
            self.y = 127.0;
        }
        self.canvas.set_pixel(self.x as u32, self.y as u32, Color::from_u32(self.color));
        if self.time_accumulator > Duration::from_secs_f64(1.0 / 60.0) {
            self.time_accumulator = Duration::new(0, 0);
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
    let (handler, mut window) = spriter::init("Dot example", 512, 512);
    let canvas = window.create_canvas(0, 0, 512, 512, 128, 128);
    let dot = Dot { x: 64.0, y: 64.0, color: 0xFF2222, canvas, run: true, time_accumulator: Duration::new(0, 0) };
    handler.run(dot, window);
}