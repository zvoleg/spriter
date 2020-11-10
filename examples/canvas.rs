extern crate spriter;

use spriter::handler::Program;
use std::time::Duration;

struct Nope();

impl Program for Nope {
    fn execute(&mut self, frame_duration: Duration) -> bool { false }
    fn is_run(&self) -> bool { todo!() }
}

fn main() {
    let (handler, mut window) = spriter::init("spriter", 512, 512);
    let mut canvas = window.create_canvas(50, 50, 120, 120, 30, 30);
    let mut color = 0xFFFFFF;
    for i in 0..30 {
        canvas.set_pixel(i, i, color);
        color <<= 1;
    }
    handler.run(Nope(), window);
}