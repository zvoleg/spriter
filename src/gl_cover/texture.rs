extern crate gl;

pub fn create_texture() -> u32 {
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
    }
    texture
}

pub fn bind_texture(texture: u32) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }
}

pub fn configure_texture() {
    unsafe {
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);
        // gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }
}

pub fn unpack_data_alignment(unpack: bool) {
    unsafe {
        let unpack = match unpack {
            true => 1,
            false => 0,
        };
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, unpack);
    }
}

pub fn set_image(width: i32, height: i32, texture: &[u8]) {
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width,
            height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            texture.as_ptr() as *const _
        );
    }
}

pub fn set_sub_image(width: i32, height: i32, texture: &[u8]) {
    unsafe {
        gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            0,
            0,
            width,
            height,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            texture.as_ptr() as *const _
        );
    }
}