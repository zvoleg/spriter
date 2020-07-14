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
}

fn main() {
    let (window, mut handler) = spriter::init("spriter", 512, 512, true);
    let s = Rc::new(RefCell::new(Tmp { value: 5 , run: true}));
    let s1 = s.clone();
    handler.add_key_handler(
        Key::S,
        move || println!("key s is pressed and s = {}", s1.borrow().get_value()));
    let s2 = s.clone();
    handler.add_key_handler(
        Key::W,
        move || println!("key s is pressed and w = {}", s2.borrow().get_value()));
    let s3 = s.clone();
    handler.add_key_handler(
        Key::D,
        move || {
            let val = s3.borrow().get_value();
            s3.borrow_mut().set_value(val + 1);
        });
    let s4 = s.clone();
    handler.add_key_handler(
        Key::A,
        move || {
            let val = s4.borrow().get_value();
            s4.borrow_mut().set_value(val - 1);
        });
    let s5 = s.clone();
    handler.add_key_handler(
        Key::Escape,
        move || {
            s5.borrow_mut().run = false;
        });
    handler.run(Rc::new(RefCell::new(window)), Some(s.clone()));
}