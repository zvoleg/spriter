#[macro_use]
extern crate spriter;

use spriter::{Key, handler::Program};
use std::time::Duration;

struct Tmp {
    value: i32,
    run: bool,
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
        if_pressed!(Key::D, {
            let val = self.get_value();
            self.set_value(val + 1);
        });
        if_pressed!(Key::A, {
            let val = self.get_value();
            self.set_value(val - 1);
        });
        if_pressed!(Key::Escape, { self.run = false });
    }
}

impl Program for Tmp {
    fn execute(&mut self, frame_duration: Duration) -> bool {
        self.handle_key_input();
        false
    }

    fn is_run(&self) -> bool {
        self.run
    }

    
}

fn main() {
    let (handler, window) = spriter::init("spriter", 512, 512);
    let s = Tmp { value: 5 , run: true};
    handler.run(s, window);
}