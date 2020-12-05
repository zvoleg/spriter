use super::{Key, Window};

use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent, ElementState};

use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard};
use std::time::{SystemTime, Duration};

lazy_static! {
    static ref PRESSED_KEYS: Mutex<HashSet<Key>> = Mutex::new(HashSet::new());
    static ref RELEASED_KEYS: Mutex<HashSet<Key>> = Mutex::new(HashSet::new());
}

fn pressed_keys() -> MutexGuard<'static, HashSet<Key>> {
    PRESSED_KEYS.lock().unwrap()
}

fn released_keys() -> MutexGuard<'static, HashSet<Key>> {
    RELEASED_KEYS.lock().unwrap()
}


pub fn key_is_pressed(key: &Key) -> bool {
    pressed_keys().contains(key)
}

pub fn key_is_released(key: &Key) -> bool {
    released_keys().contains(key)
}

pub fn try_remove_pressed_key(key: &Key) -> bool {
    pressed_keys().remove(key)
}

#[macro_export]
macro_rules! if_pressed {
    ($key: expr, $action: tt) => {
        if $crate::handler::key_is_pressed(&$key) {
            $action;
        }
    };
}

#[macro_export]
macro_rules! handle_press {
    ($key: expr, $action: tt) => {
        if $crate::handler::try_remove_pressed_key(&$key) {
            $action;
        }
    };
}

#[macro_export]
macro_rules! if_released {
    ($key: expr, $action: tt) => {
        if $crate::handler::key_is_released(&$key) {
            $action;
        }
    };
}

pub struct Handler {
    event_loop: EventLoop<()>,
}

impl Handler {
    pub fn new(event_loop: EventLoop<()>) -> Self {
        Handler { event_loop }
    }

    pub fn run<T: 'static + Program>(self, mut program: T, window: Window) -> ! {
        let mut instant = SystemTime::now();
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            match input.state {
                                ElementState::Pressed => {pressed_keys().insert(key);},
                                ElementState::Released => {
                                    pressed_keys().remove(&key);
                                    released_keys().insert(key);
                                },
                            };
                        };
                    },
                    _ => (),
                },
                Event::MainEventsCleared => {
                    let frame_duration = instant.elapsed().unwrap();
                    instant = SystemTime::now();
                    if program.execute(frame_duration) {
                        window.request_redraw();
                    }
                    if !program.is_run() {
                        *control_flow = ControlFlow::Exit;
                    }
                    released_keys().clear();
                },
                Event::RedrawRequested(_) => {
                    window.update()
                }
                _ => ()
            }
        });
    }
}

pub trait Program {
    fn execute(&mut self, frame_duration: Duration) -> bool;
    fn is_run(&self) -> bool;
}
