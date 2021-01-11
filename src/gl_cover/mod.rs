extern crate gl;

pub mod buffer;
pub mod program;
pub mod texture;

pub static ARRAY_BUFFER: u32 = gl::ARRAY_BUFFER;
pub static VERTEX_SHADER: u32 = gl::VERTEX_SHADER;
pub static FRAGMENT_SHADER: u32 = gl::FRAGMENT_SHADER;

pub fn load_proc_address<F>(mut get_proc_addr_fn: F) where F: FnMut(&'static str) -> *const core::ffi::c_void {
    gl::load_with(|ptr| get_proc_addr_fn(ptr));
}

pub fn view_port(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        gl::Viewport(x, y, width, height);
    }
}

pub fn clear_color(red: f32, grean: f32, blue: f32, alpha: f32) {
    unsafe {
        gl::ClearColor(red, grean, blue, alpha);
    }
}

pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn draw_quad() {
    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
}