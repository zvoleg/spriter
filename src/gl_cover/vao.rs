extern crate gl;

use gl::types::GLenum;

pub fn create_vao() -> u32 {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    vao
}

pub fn bind_vao(vao: u32) {
    unsafe {
        gl::BindVertexArray(vao);
    }
}

pub fn create_buffer() -> u32 {
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }
    vbo
}

pub fn bind_vbo(vbo: u32) {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    }
}

pub fn bind_ebo(ebo: u32) {
    unsafe {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    }
}

pub fn buffer_data<T>(data: &[T], typ: GLenum) {
    unsafe {
        gl::BufferData(
            typ,
            (std::mem::size_of_val(data)) as isize,
            data.as_ptr() as *const _,
            gl::DYNAMIC_DRAW
        );
    }
}

pub fn setup_attribute(id: u32, size: i32, step: usize, offset: usize) {
    unsafe {
        gl::VertexAttribPointer(
            id,
            size,
            gl::FLOAT,
            gl::FALSE,
            (step * std::mem::size_of::<f32>()) as i32,
            (offset * std::mem::size_of::<f32>()) as *const _
        );
    }
}

pub fn enable_attribute(id: u32) {
    unsafe {
        gl::EnableVertexAttribArray(id);
    }
}