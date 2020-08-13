extern crate spriter;

use spriter::Program;

struct Nope();

impl Program for Nope {
    fn run(&mut self) {}
    fn is_execute(&self) -> bool { todo!() }
    fn handle_key_input(&mut self, _: spriter::Key) { todo!() }
}

fn main() {
    let (mut window, handler) = spriter::init("spriter", 512, 512);
    let canvas = window.create_canvas(50, 50, 120, 120, 30, 30);
    let mut color = 0xFFFFFF;
    for i in 0..30 {
        canvas.borrow_mut().set_color(i, i, color);
        color <<= 1;
    }
    handler.run(window, Nope());
}