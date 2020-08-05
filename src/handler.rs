use glutin::{
    event_loop::EventLoop,
    event::KeyboardInput,
};

use super::{
    Key,
    State,
    WEvent,
    Event,
    Flow,
    Program,
};
use super::window::Window;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Mutex;

lazy_static! {
    pub static ref PRESSED_KEYS: Mutex<HashSet<Key>> = Mutex::new(HashSet::new());
}

pub fn get_pressed_keys() -> std::sync::MutexGuard<'static, HashSet<Key>> {
    PRESSED_KEYS.lock().unwrap()
}

pub struct Handler {
    event_loop: Option<EventLoop<()>>,
    key_handlers: HashMap<Key, Box<dyn FnMut() + 'static>>,
    auto_frame_update: bool,
    is_run: bool,
}

impl Handler {
    pub fn new(event_loop: Option<EventLoop<()>>, auto_frame_update: bool) -> Handler {
        let key_handlers = HashMap::new();
        Handler { 
            event_loop,
            key_handlers,
            auto_frame_update,
            is_run: true,
        }
    }

    pub fn add_key_handler<F>(&mut self, key: Key, func: F)
    where F: FnMut() + 'static {
        self.key_handlers.insert(key, Box::new(func));
    }

    fn handle_keys(&mut self) {
        for key in get_pressed_keys().iter() {
            if self.key_handlers.contains_key(key) {
                self.key_handlers.get_mut(key).unwrap()();
            }
        }
    }

    fn handle_program_keys(&mut self, program: &mut dyn Program) {
        for key in get_pressed_keys().iter() {
            program.handle_key_input(*key);
        }
    }

    fn check_key(&mut self, input: KeyboardInput) {
        if let Some(key) = input.virtual_keycode {
            if input.state == State::Pressed {
                get_pressed_keys().insert(key);
            } else {
                get_pressed_keys().remove(&key);
            }
        }
    }

    pub fn run(mut self, window: Rc<RefCell<Window>>, program: Option<Rc<RefCell<dyn Program>>>) -> !  {
        use std::time::{Instant, Duration};

        let event_loop = self.event_loop.take().unwrap();
        let mut frames = 0;
        let mut instant = Instant::now();
        event_loop.run(move |events, _, control_flow| {
            *control_flow = Flow::Poll;
            match events {
                Event::WindowEvent { event: WEvent::KeyboardInput { input, .. } , .. } => self.check_key(input),
                Event::WindowEvent { event: WEvent::CloseRequested, .. } => {
                    self.is_run = false;
                }
                _ => (),
            }
            self.handle_keys();
            if let Some(p) = &program {
                if self.is_run {
                    self.is_run = p.borrow().is_execute();
                }
                self.handle_program_keys(&mut *p.borrow_mut());
                p.borrow_mut().run();
            }
            if !self.is_run {
                *control_flow = Flow::Exit;
            }
            if self.auto_frame_update {
                window.borrow_mut().swap_buffers();
            }
            if instant.elapsed() >= Duration::from_secs(1) {
                window.borrow_mut().context.window().set_title(&format!("fps: {}", frames));
                frames = 0;
                instant = Instant::now();
            } else {
                frames += 1;
            }
        })
    }
}
