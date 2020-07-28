extern crate glutin;
extern crate gl;

pub use glutin::{
    event_loop::ControlFlow as Flow,
    event::Event,
    event::WindowEvent as WEvent,
    event::VirtualKeyCode as Key,
    event::ElementState as State,
};

use glutin::{
    event_loop::EventLoop,
    window::WindowBuilder,
    ContextBuilder,
    dpi::PhysicalSize,
};

mod gl_cover;

pub mod handler;
pub mod window;

use handler::Handler;
use window::Window;

pub fn init(title: &str, width: u32, height: u32, auto_frame_update: bool) -> (Window, Handler) {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(width, height));
    let window_context = ContextBuilder::new()
        .with_gl(glutin::GlRequest::Latest)
        .with_gl_profile(glutin::GlProfile::Core)
        .build_windowed(window_builder, &event_loop)
        .unwrap();
    let window_context = unsafe { window_context.make_current().unwrap() };
    gl::load_with(|ptr| window_context.get_proc_address(ptr) as *const _);
    
    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);
        gl::ClearColor(0.5f32, 0.5, 0.65, 1.0);
    }

    let window = Window::new(window_context, width, height);
    let handler = Handler::new(Some(event_loop), auto_frame_update);
    (window, handler)
}

pub trait Program {
    fn is_execute(&self) -> bool;
    fn run(&mut self);
    fn handle_key_input(&mut self, key: Key);
}