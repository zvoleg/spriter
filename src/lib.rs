extern crate glutin;
#[macro_use]
extern crate lazy_static;

pub mod handler;
pub mod window;
pub use handler::Program;
pub use window::{Canvas, Color};
pub use glutin::event::VirtualKeyCode as Key;

mod gl_cover;

use handler::Handler;
use window::Window;

use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::dpi::PhysicalSize;

pub fn init(title: &str, width: u32, height: u32) -> (Handler, Window) {
    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(width, height));

    #[cfg(target_os = "windows")]
    let window_builder = {
        use glutin::platform::windows::WindowBuilderExtWindows;
        window_builder.with_drag_and_drop(false)
    };

    let windowed_context = ContextBuilder::new()
        .with_gl(glutin::GlRequest::Latest)
        .with_gl_profile(glutin::GlProfile::Core)
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    (Handler::new(event_loop), Window::new(windowed_context, width, height))
}

trait Render {
    fn update(&self);
    fn request_redraw(&self);
}