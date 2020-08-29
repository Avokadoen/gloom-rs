#[allow(dead_code)]
pub enum ShaderType {
    Vertex,
    Fragment,
    TessellationControl,
    TessellationEvaluation,
    Geometry,
}

impl ShaderType {
    pub fn from_ext(ext: &std::ffi::OsStr) -> Result<ShaderType, String> {
        match ext.to_str().expect("Failed to read extension") {
            "vert" => { Ok(ShaderType::Vertex) },
            "frag" => { Ok(ShaderType::Fragment) },
            "tcs"  => { Ok(ShaderType::TessellationControl) },
            "tes"  => { Ok(ShaderType::TessellationEvaluation) },
            "geom" => { Ok(ShaderType::Geometry) },
            e => { Err(e.to_string()) },
        }
    }
}

impl Into<gl::types::GLenum> for ShaderType {
    fn into(self) -> gl::types::GLenum {
        match self {
            ShaderType::Vertex                  => { gl::VERTEX_SHADER          },
            ShaderType::Fragment                => { gl::FRAGMENT_SHADER        },
            ShaderType::TessellationControl     => { gl::TESS_CONTROL_SHADER    },
            ShaderType::TessellationEvaluation  => { gl::TESS_EVALUATION_SHADER } ,
            ShaderType::Geometry                => { gl::GEOMETRY_SHADER        },
        }
    }
}

