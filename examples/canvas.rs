extern crate spriter;

use spriter::{Program, Color};
use std::time::Duration;

struct Nope();

impl Program for Nope {
    fn execute(&mut self, _frame_duration: Duration) -> bool { true }
    fn is_run(&self) -> bool { true }
}

fn main() {
    let (handler, mut window) = spriter::init("spriter", 512, 512);
    let mut canvas = window.create_canvas(50, 50, 120, 120, 30, 30);
    let mut color = 0xFFFFFF;
    for i in 0..30 {
        canvas.set_pixel(i, i, Color::from_u32(color));
        color <<= 1;
    }
    handler.run(Nope(), window);
}