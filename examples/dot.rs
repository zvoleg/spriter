#[macro_use]
extern crate spriter;

use spriter::{Canvas, Color, Key};

use std::time::Duration;

struct Dot {
    x: f32,
    y: f32,
    color: u32,
    canvas: Canvas,
    time_accumulator: Duration,
}

fn main() {
    let (handler, mut window) = spriter::init("Dot example", 512, 512);
    let mut canvas = window.create_canvas(0, 0, 512, 512, 64, 64);
    canvas.set_clear_color(Color::from_u32(0x4422AA));
    let mut dot = Dot { x: 31.0, y: 31.0, color: 0x4422AA, canvas, time_accumulator: Duration::new(0, 0) };
    handler.run(window, move |frame_duration| {
        dot.canvas.clear();
        let new_time = dot.time_accumulator.checked_add(frame_duration).unwrap();
        dot.time_accumulator = new_time;
        if_pressed!(Key::Space, {dot.color = 0xAA9911});
        if_holded!(Key::W, {dot.y -= 0.001;});
        if_holded!(Key::S, {dot.y += 0.001;});
        if_holded!(Key::A, {dot.x -= 0.001;});
        if_holded!(Key::D, {dot.x += 0.001;});
        if_pressed!(Key::Space, {
            dot.color += 0x131313;
            dot.color &= 0xFFFFFF;
        });
        if_pressed!(Key::Escape, { spriter::program_stop() });
        if dot.x < 0.0 {
            dot.x = 0.0;
        }
        if dot.x > 63.0 {
            dot.x = 63.0;
        }
        if dot.y < 0.0 {
            dot.y = 0.0;
        }
        if dot.y > 63.0 {
            dot.y = 63.0;
        }
        dot.canvas.set_pixel(dot.x as i32, dot.y as i32, Color::from_u32(dot.color)).unwrap();
        if dot.time_accumulator > Duration::from_secs_f64(1.0 / 30.0) {
            dot.time_accumulator = Duration::new(0, 0);
            true
        } else {
            false
        }
    });
}