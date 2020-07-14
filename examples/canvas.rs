extern crate spriter;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let (mut window, handler) = spriter::init("spriter", 512, 512, true);
    let canvas = window.create_canvas(50, 50, 120, 120, 30, 30);
    let mut color = 0xFFFFFF;
    for i in 0..30 {
        canvas.borrow_mut().set_color(i, i, color);
        color <<= 1;
    }
    handler.run(Rc::new(RefCell::new(window)), None);
}