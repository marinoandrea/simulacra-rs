use std::{ffi::CString, ptr, str};

pub trait Shader<'a> {
    fn new(vs: &'a str, fs: &'a str) -> Self;
    fn bind(&self);
    fn unbind(&self);
}

pub struct OpenGLShader {
    program_id: u32,
}

impl<'a> Shader<'a> for OpenGLShader {
    fn new(vs: &'a str, fs: &'a str) -> Self {
        let vs_id = OpenGLShader::compile_shader(vs, gl::VERTEX_SHADER);
        let fs_id = OpenGLShader::compile_shader(fs, gl::FRAGMENT_SHADER);
        let program_id = OpenGLShader::link_program(vs_id, fs_id);

        Self { program_id }
    }

    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl Drop for OpenGLShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

impl OpenGLShader {
    pub fn get_program_id(&self) -> u32 {
        self.program_id
    }

    fn compile_shader(src: &str, ty: u32) -> u32 {
        use gl::types::*;

        let shader;
        unsafe {
            shader = gl::CreateShader(ty);
            // Attempt to compile the shader
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ShaderInfoLog not valid utf8")
                );
            }
        }
        shader
    }

    fn link_program(vs: u32, fs: u32) -> u32 {
        use gl::types::*;

        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            gl::DetachShader(program, vs);
            gl::DetachShader(program, fs);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ProgramInfoLog not valid utf8")
                );
            }
            program
        }
    }
}
