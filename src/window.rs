use glutin::{ContextWrapper, PossiblyCurrent};

use super::gl_cover as gl;
use super::Render;

pub struct Window {
    context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    unfrm_projection: i32,
    unfrm_model: i32,
    canvases: Vec<CanvasAtributes>,
    projection_matrix: [f32; 16],
}

impl Window {
    pub fn new(context: ContextWrapper<PossiblyCurrent, glutin::window::Window>, width: u32, height: u32) -> Self {
        gl::load_proc_address(|ptr| context.get_proc_address(ptr));
        gl::view_port(0, 0, width as i32, height as i32);
        gl::clear_color(0.3, 0.3, 0.7, 1.0);
        
        let program = setup_quad_program();
        let unfrm_projection = gl::program::uniform_location(program, "projection");
        let unfrm_model = gl::program::uniform_location(program, "model");

        let canvases = Vec::new();
        let width = width as f32;
        let height = height as f32;
        let projection_matrix = [
            2.0 / width, 0.0, 0.0, 0.0,
            0.0, 2.0 / -height, 0.0, 0.0,
            0.0, 0.0, -2.0 / (10.0 - 0.1), 0.0,
            -1.0, 1.0, -1.0202, 1.0
        ];
        Window { context, unfrm_projection, unfrm_model, canvases, projection_matrix }
    }

    pub fn create_canvas(&mut self, x: u32, y: u32, width: u32, height: u32, t_width: u32, t_height: u32) -> Canvas {
        let (canvas, canvas_atributes) = Canvas::new(x, y, width, height, t_width, t_height);
        self.canvases.push(canvas_atributes);
        canvas
    }
}

impl Render for Window {
    fn update(&mut self) {
        gl::clear();
        for canvas in self.canvases.iter_mut() {
            gl::buffer::bind_buffer(canvas.pbo, gl::PIXEL_UNPACK_BUFFER);
            let mem_ptr = gl::map_buffer();
            unsafe {
                std::ptr::copy_nonoverlapping(canvas.texture_buffer_ptr, mem_ptr, canvas.buffer_size);
            }
            gl::unmap_buffer();

            gl::texture::bind_texture(canvas.texture);
            gl::texture::texture_subimage(canvas.t_width, canvas.t_height, std::ptr::null());
            gl::buffer::bind_buffer(0, gl::PIXEL_UNPACK_BUFFER);

            gl::program::uniform_matrix(self.unfrm_model, &canvas.model_matrix);
            gl::program::uniform_matrix(self.unfrm_projection, &self.projection_matrix);
            gl::draw_quad();
        }
        self.context.swap_buffers().unwrap();
    }

    fn request_redraw(&self) {
        self.context.window().request_redraw();
    }
}

pub struct CanvasAtributes {
    pbo: u32,
    texture: u32,
    model_matrix: [f32; 16],
    texture_buffer_ptr: *const std::ffi::c_void,
    buffer_size: usize,
    t_width: u32,
    t_height: u32,
}

#[derive(Copy, Clone)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn from_u32(color: u32) -> Self {
        let r = (color >> 16) as u8;
        let g = (color >> 8) as u8;
        let b = color as u8;
        Self::new(r, g, b)
    }
}

pub struct Canvas {
    texture_buffer: Vec<Color>,
    color: Color,
    t_width: u32,
    t_height: u32,
}

impl Canvas {
    fn new(x: u32, y: u32, width: u32, height: u32, t_width: u32, t_height: u32) -> (Self, CanvasAtributes) {
        let x = x as f32;
        let y = y as f32;
        let width = width as f32;
        let height = height as f32;
        let model_matrix = [
            width, 0.0, 0.0, 0.0,
            0.0, height, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            x, y, -1.0, 1.0,
        ];
        let color = Color::new(0x80, 0xA0, 0x80);
        let texture_buffer = vec![color; (t_width * t_height) as usize];
        let texture_buffer_ptr = texture_buffer.as_ptr() as *const _;
        let buffer_size = std::mem::size_of_val(&(*texture_buffer));

        let texture = gl::texture::create_texture();
        gl::texture::bind_texture(texture);
        gl::texture::setup_texture();
        gl::texture::unpack_data_alignment();
        gl::texture::texture_image(t_width, t_height, texture_buffer_ptr);

        let pbo = gl::buffer::generate_buffer();
        gl::buffer::bind_buffer(pbo, gl::PIXEL_UNPACK_BUFFER);
        gl::buffer::raw_buffer_data(gl::PIXEL_UNPACK_BUFFER, buffer_size, std::ptr::null(), gl::STREAM_DRAW);
        gl::buffer::bind_buffer(0, gl::PIXEL_UNPACK_BUFFER);

        let canvas = Canvas { texture_buffer, color, t_width, t_height };
        let canvas_atributes = CanvasAtributes { pbo, texture, model_matrix, texture_buffer_ptr, buffer_size, t_width, t_height };

        (canvas, canvas_atributes)
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) -> Result<(), String> {
        if x < 0 || self.t_width as i32 <= x || y < 0 || self.t_height as i32 <= y {
            return Err(format!("out of bounds: x={}; y={}", x, y));
        }
        let x = x as u32;
        let y = y as u32;
        let idx = (self.t_width * y + x) as usize;
        unsafe { *self.texture_buffer.get_unchecked_mut(idx) = color }; 
        Ok(())
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn clear(&mut self) {
        for pixel in &mut self.texture_buffer {
            *pixel = self.color;
        }
    }
}

fn setup_quad_program() -> u32 {
    let vao = gl::buffer::generate_vertex_array();
    gl::buffer::bind_array(vao);

    let vertices = [
        0.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 1.0, 0.0,
        0.0, 1.0, 0.0, 1.0,
        1.0, 0.0, 1.0, 0.0,
        1.0, 1.0, 1.0, 1.0,
        0.0, 1.0, 0.0, 1.0f32
    ];

    let vbo = gl::buffer::generate_buffer();
    gl::buffer::bind_buffer(vbo, gl::ARRAY_BUFFER);
    gl::buffer::buffer_data(&vertices, gl::ARRAY_BUFFER);

    let vertex_shader = create_compiled_shader(VERTEX_SHADER, gl::VERTEX_SHADER);
    let fragment_shader = create_compiled_shader(FRAGMENT_SHADER, gl::FRAGMENT_SHADER);

    let program = gl::program::create_program();
    gl::program::attach_shader(program, vertex_shader);
    gl::program::attach_shader(program, fragment_shader);
    gl::program::link_program(program);

    gl::program::use_program(program);
    
    gl::program::delete_shader(vertex_shader);
    gl::program::delete_shader(fragment_shader);

    gl::program::vertex_attrib_pointer(0, 2, 4, 0);
    gl::program::vertex_attrib_pointer(1, 2, 4, 2);
    gl::program::enable_attribute(0);
    gl::program::enable_attribute(1);

    program
}

fn create_compiled_shader(shader_src: &str, shader_type: u32) -> u32 {
    let shader = gl::program::create_shader(shader_type);
    gl::program::shader_source(shader, &shader_src);
    gl::program::compile_shader(shader);
    shader
}

static VERTEX_SHADER: &str = r"
#version 330 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 texture_position;

out vec2 texture_cord;

uniform mat4 projection;
uniform mat4 model;

void main() {
    gl_Position = projection * model * vec4(position, 0.0, 1.0);
    texture_cord = texture_position;
}
";

static FRAGMENT_SHADER: &str = r"
#version 330 core

in vec2 texture_cord;
out vec4 color;

uniform sampler2D texture_img;

void main() {
    color = texture(texture_img, texture_cord);
}
";