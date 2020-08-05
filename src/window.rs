extern crate gl as system_gl;

use glutin::{
    PossiblyCurrent,
    ContextWrapper,
    window::Window as GlutinWindow,
};

use super::gl_cover as gl;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Window {
    pub context: ContextWrapper<PossiblyCurrent, GlutinWindow>,
    polygon: Polygon,
    projection: [f32; 16],
    canvases: Vec<Rc<RefCell<Canvas>>>,
}

impl Window {
    pub fn new(context: ContextWrapper<PossiblyCurrent, GlutinWindow>, width: u32, height: u32) -> Window {
        let polygon = Polygon::new();
        let width = width as f32;
        let height = height as f32;
        let projection = [
            2. / (width - 0.),               0.,                                0.,                           0.,
            0.,                              2. / (0. - height),                0.,                           0.,
            0.,                              0.,                                -(2. / (10. - 0.)),           0.,
            -((width + 0.) / (width - 0.)),  -((0. + height) / (0. - height)),  -((10. + 0.) / (10. - 0.)),   1.
        ];
        let canvases = Vec::new();
        Window { 
            context,
            polygon,
            projection,
            canvases,
        }
    }

    pub fn create_canvas(&mut self, x: u32, y: u32, width: u32, height: u32, texture_width: u32, texture_height: u32) -> Rc<RefCell<Canvas>> {
        let canvas = Rc::new(RefCell::new(Canvas::new(x, y, width, height, texture_width, texture_height, self.polygon.vbo, self.polygon.ebo)));
        self.canvases.push(canvas.clone());
        canvas
    }

    pub fn swap_buffers(&mut self) {

        unsafe {
            system_gl::Clear(system_gl::COLOR_BUFFER_BIT);
        }
        for canvas in self.canvases.iter() {
            let canvas = canvas.borrow();
            gl::vao::bind_vao(canvas.id);
            gl::texture::bind_texture(canvas.texture);
            gl::texture::set_sub_image(canvas.texture_width as i32, canvas.texture_height as i32, &canvas.buffer);

            unsafe {
                system_gl::UniformMatrix4fv(
                    self.polygon.model_location,
                    1,
                    system_gl::FALSE,
                    canvas.model_matrix.as_ptr()
                );
                system_gl::UniformMatrix4fv(
                    self.polygon.projection_location,
                    1,
                    system_gl::FALSE,
                    self.projection.as_ptr()
                );
                system_gl::DrawElements(
                    system_gl::TRIANGLES,
                    6,
                    system_gl::UNSIGNED_INT,
                    std::ptr::null()
                );
            }
        }
        self.context.swap_buffers().unwrap();
    }
}

pub struct Canvas {
    id: u32,
    texture: u32,
    texture_width: u32, 
    texture_height: u32,
    model_matrix: [f32; 16],
    buffer: Vec<u8>,
}

impl Canvas {
    fn new(x: u32, y: u32, width: u32, height: u32, texture_width: u32, texture_height: u32, vbo: u32, ebo: u32) -> Canvas {
        let id = gl::vao::create_vao();

        gl::vao::bind_vao(id);
        gl::vao::bind_vbo(vbo);
        gl::vao::bind_ebo(ebo);

        let width = width as f32;
        let height = height as f32;
        let x = x as f32;
        let y = y as f32;
        let model_matrix = [
            width,  0.,     0.,     0.,
            0.,     height, 0.,     0.,
            0.,     0.,     1.,     0.,
            x,      y,      -1.,    1.
        ];

        let buffer = vec![0x50; (texture_width * texture_height * 3) as usize];
        let texture = gl::texture::create_texture();
        gl::texture::bind_texture(texture);
        gl::texture::unpack_data_alignment(true);
        gl::texture::configure_texture();
        gl::texture::set_image(texture_width as i32, texture_height as i32, &buffer);

        gl::vao::setup_attribute(0, 2, 4, 0);
        gl::vao::setup_attribute(1, 2, 4, 2);
        gl::vao::enable_attribute(0);
        gl::vao::enable_attribute(1);

        Canvas {
            id,
            texture,
            texture_width,
            texture_height,
            model_matrix,
            buffer,
        }
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: u32) {
        let r = (color >> 16) as u8;
        let g = (color >> 8) as u8;
        let b = color as u8;
        let idx = ((y * self.texture_width as usize) + x) * 3;
        self.buffer[idx] = r;
        self.buffer[idx + 1] = g;
        self.buffer[idx + 2] = b;
    }

    pub fn set_image(&mut self, image: &[u8]) {
        self.buffer.clone_from_slice(image);
    }
}

struct Polygon {
    vbo: u32,
    ebo: u32,
    model_location: i32,
    projection_location: i32,
}

impl Polygon {
    fn new() -> Polygon {
        let points = [
            0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 1.0f32,
        ];
        let vbo = gl::vao::create_buffer();
        gl::vao::bind_vbo(vbo);
        gl::vao::buffer_data(&points, system_gl::ARRAY_BUFFER);
        
        let indices = [
            0, 1, 2,
            2, 1, 3u32
        ];
        let ebo = gl::vao::create_buffer();
        gl::vao::bind_ebo(ebo);
        gl::vao::buffer_data(&indices, system_gl::ELEMENT_ARRAY_BUFFER);

        use std::str::FromStr;

        let vertex_src = String::from_str(r"
            #version 330 core

            layout (location = 0) in vec2 vertex;
            layout (location = 1) in vec2 texture_coordinates;

            out vec2 texture_position;

            uniform mat4 model;
            uniform mat4 projection;

            void main() {
                gl_Position = projection * model * vec4(vertex, 0.0, 1.0);
                texture_position = texture_coordinates;
            }
        ").unwrap();
        let fragment_src = String::from_str(r"
            #version 330 core

            in vec2 texture_position;

            out vec4 color;

            uniform sampler2D texture_img;

            void main() {
                color = texture(texture_img, texture_position);
            }
        ").unwrap();
        let vertex_shader = gl::program::create_shader(system_gl::VERTEX_SHADER);
        let fragment_shader = gl::program::create_shader(system_gl::FRAGMENT_SHADER);
        gl::program::shader_src(vertex_shader, &vertex_src);
        gl::program::shader_src(fragment_shader, &fragment_src);
        gl::program::compile_shader(vertex_shader);
        gl::program::compile_shader(fragment_shader);
        
        let program = gl::program::create_program();
        gl::program::attach_shader(program, vertex_shader);
        gl::program::attach_shader(program, fragment_shader);
        gl::program::link_program(program);
        gl::program::use_program(program);

        gl::program::delete_shader(vertex_shader);
        gl::program::delete_shader(fragment_shader);

        let model_location = gl::program::get_uniform(program, "model");
        let projection_location = gl::program::get_uniform(program, "projection");

        gl::vao::setup_attribute(0, 2, 4, 0);
        gl::vao::setup_attribute(1, 2, 4, 2);
        gl::vao::enable_attribute(0);
        gl::vao::enable_attribute(1);

        Polygon { 
            vbo,
            ebo,
            model_location,
            projection_location,
        }
    }
}
