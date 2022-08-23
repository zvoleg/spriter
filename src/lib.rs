extern crate glutin;
#[macro_use]
extern crate lazy_static;

pub mod window;
pub use window::{Canvas, Color};
pub use glutin::event::VirtualKeyCode as Key;

mod gl_cover;

use window::Window;

use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent, KeyboardInput, ElementState};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::dpi::PhysicalSize;

use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, SystemTime};

static mut PROGRAM_STATE: ProgramState = ProgramState::Stop;

lazy_static! {
    static ref PRESSED_KEYS: Mutex<HashSet<Key>> = Mutex::new(HashSet::new());
    static ref HOLDED_KEYS: Mutex<HashSet<Key>> = Mutex::new(HashSet::new());
    static ref RELEASED_KEYS: Mutex<HashSet<Key>> = Mutex::new(HashSet::new());
}

pub fn init(title: &str, width: u32, height: u32) -> (Runner, Window) {
    #[cfg(target_os = "windows")]
    {
        extern crate winapi;
        use winapi::um::combaseapi::CoInitializeEx;
        use winapi::um::objbase::COINIT_MULTITHREADED;
        unsafe {
            CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED);
        }
    }

    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(width, height));

    let windowed_context = ContextBuilder::new()
        .with_gl(glutin::GlRequest::Latest)
        .with_gl_profile(glutin::GlProfile::Core)
        .build_windowed(window_builder, &event_loop)
        .expect("Can't create context wrapper");

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let runner = Runner { event_loop };
    (runner, Window::new(windowed_context, width, height))
}

pub struct Runner {
    event_loop: EventLoop<()>,
}

#[derive(Copy, Clone)]
enum ProgramState {
    Run,
    Stop,
}

impl Runner {
    pub fn run<F>(self, mut window: Window, mut user_program: F) -> !
        where F: 'static + FnMut(Duration) -> bool 
    {
        program_run();
        let mut instant = SystemTime::now();
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => Runner::keyboard_input_update(input),
                    _ => (),
                },
                Event::MainEventsCleared => {
                    Runner::execute_main_program(
                        &mut instant,
                        control_flow,
                        &window,
                        &mut user_program);                 
                },
                Event::RedrawRequested(_) => {
                    window.update()
                }
                _ => ()
            }
        })
    }

    fn keyboard_input_update(input: KeyboardInput) {
        if let Some(key) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => {
                    pressed_keys().insert(key);
                    holded_keys().insert(key);
                },
                ElementState::Released => {
                    holded_keys().remove(&key);
                    released_keys().insert(key);
                },
            };
        };
    }

    fn execute_main_program<F>(
        time_instant: &mut SystemTime,
        control_flow: &mut ControlFlow,
        window: &Window, 
        user_program: &mut F)
        where F: FnMut(Duration) -> bool
    {
        let frame_duration = time_instant.elapsed().unwrap();
        *time_instant = SystemTime::now();
        let need_window_update = match program_state() {
            ProgramState::Run => {
                user_program(frame_duration)
            },
            ProgramState::Stop => {
                *control_flow = ControlFlow::Exit;
                false
            },
        };
        pressed_keys().clear();
        released_keys().clear();
        if need_window_update{
            window.request_redraw();
        }
    }
}

fn program_state() -> ProgramState {
    unsafe {
        PROGRAM_STATE
    }
}

fn program_run() {
    unsafe {
        PROGRAM_STATE = ProgramState::Run;
    }
}

pub fn program_stop() {
    unsafe {
        PROGRAM_STATE = ProgramState::Stop;
    }
}

fn pressed_keys() -> MutexGuard<'static, HashSet<Key>> {
    PRESSED_KEYS.lock().unwrap()
}

fn holded_keys() -> MutexGuard<'static, HashSet<Key>> {
    HOLDED_KEYS.lock().unwrap()
}

fn released_keys() -> MutexGuard<'static, HashSet<Key>> {
    RELEASED_KEYS.lock().unwrap()
}

pub fn key_is_pressed(key: &Key) -> bool {
    pressed_keys().contains(key)
}

pub fn key_is_holded(key: &Key) -> bool {
    holded_keys().contains(key)
}

pub fn key_is_released(key: &Key) -> bool {
    released_keys().remove(key)
}

#[macro_export]
macro_rules! if_pressed {
    ($key: expr, $action: tt) => {
        if $crate::key_is_pressed(&$key) {
            $action;
        }
    };
}

#[macro_export]
macro_rules! if_holded {
    ($key: expr, $action: tt) => {
        if $crate::key_is_holded(&$key) {
            $action;
        }
    };
}

#[macro_export]
macro_rules! if_released {
    ($key: expr, $action: tt) => {
        if $crate::key_is_released(&$key) {
            $action;
        }
    };
}

trait Render {
    fn update(&mut self);
    fn request_redraw(&self);
}