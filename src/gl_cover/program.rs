use gl::types::*;

use std::ffi::CString;

pub fn create_shader(type_: GLenum) -> GLuint {
    unsafe {
        gl::CreateShader(type_)
    }
}

pub fn shader_source(shader: GLuint, src: &str) {
    unsafe {
        gl::ShaderSource(
            shader,
            1,
            &CString::new(src).unwrap().as_ptr(),
            std::ptr::null()
        );
    }
}

pub fn copmpile_shader(shader: GLuint) {
    unsafe {
        gl::CompileShader(shader);
    }
}

pub fn create_program() -> GLuint {
    unsafe {
        gl::CreateProgram()
    }
}

pub fn attach_shader(program: GLuint, shader: GLuint) {
    unsafe {
        gl::AttachShader(program, shader);
    }
}

pub fn link_program(program: GLuint) {
    unsafe {
        gl::LinkProgram(program);
    }
}

pub fn delete_shader(shader: GLuint) {
    unsafe {
        gl::DeleteShader(shader);
    }
}

pub fn use_program(program: GLuint) {
    unsafe {
        gl::UseProgram(program);
    }
}

pub fn vertex_attrib_pointer(index: GLuint, size: GLint, step: usize, offset: usize) {
    unsafe {
        gl::VertexAttribPointer(
            index,
            size,
            gl::FLOAT,
            gl::FALSE,
            (step * std::mem::size_of::<f32>()) as i32,
            (offset * std::mem::size_of::<f32>()) as *const _
        );
    }
}

pub fn enable_attribute(index: GLuint) {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }
}

pub fn uniform_location(program: GLuint, uniform: &str) -> GLint {
    unsafe {
        gl::GetUniformLocation(program, CString::new(uniform).unwrap().as_ptr())
    }
}

pub fn uniform_matrix(uniform_location: GLint, matrix: &[f32]) {
    unsafe {
        gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, matrix.as_ptr());
    }
}