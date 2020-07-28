extern crate spriter;

use spriter::{Key, Program};
use std::rc::Rc;
use std::cell::RefCell;

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
}

impl Program for Tmp {
    fn is_execute(&self) -> bool {
        self.run
    }

    fn run(&mut self) {
        
    }

    fn handle_key_input(&mut self, key: Key) {
        match key {
            Key::S => println!("key s is pressed and s = {}", self.get_value()),
            Key::W => println!("key s is pressed and w = {}", self.get_value()),
            Key::D => {
                let val = self.get_value();
                self.set_value(val + 1);
            }
            Key::A => {
                let val = self.get_value();
                self.set_value(val - 1);
            }
            Key::Escape => self.run = false,
            _ => (),
        }
    }
}

fn main() {
    let (window, handler) = spriter::init("spriter", 512, 512, true);
    let s = Rc::new(RefCell::new(Tmp { value: 5 , run: true}));
    handler.run(Rc::new(RefCell::new(window)), Some(s));
}