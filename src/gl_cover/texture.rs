use gl::types::*;

pub fn create_texture() -> GLuint {
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
    }
    texture
}

pub fn bind_texture(texture: GLuint) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }
}

pub fn texture_image(width: u32, height: u32, buffer: *const std::ffi::c_void) {
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width as i32,
            height as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            buffer
        );
    }
}

pub fn texture_subimage(width: u32, height: u32, buffer: *const std::ffi::c_void) {
    unsafe {
        gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            0,
            0,
            width as i32,
            height as i32,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            buffer
        );
    }
}

pub fn unpack_data_alignment() {
    unsafe {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
    }
}

pub fn setup_texture() {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }
}
