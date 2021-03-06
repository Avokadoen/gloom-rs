
use gl::types::GLenum;
use std::{fmt, ffi};

pub enum ShaderProgramError {
    GlUniform(GlUniformError),
    UniformNotFound,
    CStr(ffi::NulError),
} 

impl fmt::Display for ShaderProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShaderProgramError::GlUniform(e) => e.fmt(f),
            ShaderProgramError::UniformNotFound => write!(f, "Failed to find uniform value on gl::GetUniformLocation"),
            ShaderProgramError::CStr(e) => e.fmt(f)
        }
    }
}

// TODO: convert file to module
// TODO: Split into 3 errors and have an upper error type see: https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html 
pub struct GlUniformError {
    error_code: GLenum,
}

impl fmt::Display for GlUniformError {
    /*
        GL_INVALID_VALUE is generated if program is not a value generated by OpenGL.
        GL_INVALID_OPERATION is generated if program is not a program object.
        GL_INVALID_OPERATION is generated if program has not been successfully linked.
    */
    // TODO: less shitty error msgs
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_code {
            gl::INVALID_VALUE => write!(f, "gl::INVALID_VALUE on uniform operation"),
            gl::INVALID_OPERATION => write!(f, "gl::INVALID_OPERATION on uniform operation"),
            i => write!(f, "unaccounted {} for error on gl::GetUniformLocation", i)
        }
    }
}

impl GlUniformError {
    pub fn new(error_code: GLenum) -> Self {
        GlUniformError {
            error_code
        }
    }
}