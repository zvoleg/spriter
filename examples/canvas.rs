extern crate spriter;

use spriter::Color;

fn main() {
    let (handler, mut window) = spriter::init("spriter", 512, 512);
    let mut canvas = window.create_canvas(50, 50, 120, 120, 30, 30);
    let mut color = 0xFFFFFF;
    for i in 0..30 {
        canvas.set_pixel(i, i, Color::from_u32(color)).unwrap();
        color <<= 1;
    }
    handler.run(window, move|_| {false});
}