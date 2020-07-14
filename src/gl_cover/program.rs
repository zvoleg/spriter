extern crate gl;

use gl::types::GLenum;
use std::ffi::CString;

pub fn create_shader(typ: GLenum) -> u32 {
    unsafe {
        gl::CreateShader(typ)
    }
}

pub fn shader_src(shader: u32, src: &str) {
    unsafe {
        gl::ShaderSource(
            shader,
            1,
            &CString::new(src).unwrap().as_ptr(),
            std::ptr::null()
        );
    }
}

pub fn compile_shader(shader: u32) {
    unsafe {
        gl::CompileShader(shader);
    }
}

pub fn delete_shader(shader: u32) {
    unsafe {
        gl::DeleteShader(shader);
    }
}

pub fn create_program() -> u32 {
    unsafe {
        gl::CreateProgram()
    }
}

pub fn attach_shader(program: u32, shader: u32) {
    unsafe {
        gl::AttachShader(program, shader);
    }
}

pub fn link_program(program: u32) {
    unsafe {
        gl::LinkProgram(program);
    }
}

pub fn use_program(program: u32) {
    unsafe {
        gl::UseProgram(program);
    }
}

pub fn get_uniform(program: u32, uniform: &str) -> i32 {
    unsafe {
        gl::GetUniformLocation(
            program,
            CString::new(uniform).unwrap().as_ptr()
        )
    }
}
