#[macro_use]
extern crate spriter;

use spriter::Key;

struct Tmp {
    value: i32,
}

impl Tmp {
    fn get_value(&self) -> i32 {
        self.value
    }
    
    fn set_value(&mut self, i: i32) {
        self.value = i;
    }

    fn handle_key_input(&mut self) {
        if_pressed!(Key::S, { println!("key s is pressed and s = {}", self.get_value()) });
        if_pressed!(Key::W, { println!("key s is pressed and w = {}", self.get_value()) });
        if_holded!(Key::D, {
            let val = self.get_value();
            self.set_value(val + 1);
        });
        if_holded!(Key::A, {
            let val = self.get_value();
            self.set_value(val - 1);
        });
        if_pressed!(Key::Escape, { spriter::program_stop() });
    }
}

fn main() {
    let (handler, window) = spriter::init("spriter", 512, 512);
    let mut s = Tmp { value: 5 };
    handler.run(window, move |_frame_duration| {
        s.handle_key_input();
        false
    });
}