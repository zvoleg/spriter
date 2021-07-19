use gl::types::*;

pub fn generate_buffer() -> GLuint {
    let mut buffer = 0;
    unsafe {
        gl::GenBuffers(1, &mut buffer);
    }
    buffer
}

pub fn bind_buffer(buffer: GLuint, type_: GLenum) {
    unsafe {
        gl::BindBuffer(type_, buffer);
    }
}

pub fn raw_buffer_data(type_: GLenum, size: usize, data_ptr: *const std::ffi::c_void, usage: GLenum) {
    unsafe {
        gl::BufferData(
            type_,
            size as isize,
            data_ptr,
            usage
        )
    }
}

pub fn buffer_data<T>(data: &[T], type_: GLenum) {
    unsafe {
        gl::BufferData(
            type_,
            std::mem::size_of_val(data) as isize,
            data.as_ptr() as *const _,
            gl::DYNAMIC_DRAW
        );
    }
}

pub fn generate_vertex_array() -> GLuint {
    let mut array = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut array);
    }
    array
}

pub fn bind_array(array: GLuint) {
    unsafe {
        gl::BindVertexArray(array);
    }
}