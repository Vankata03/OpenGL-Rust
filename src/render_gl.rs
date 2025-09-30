//! Utility helpers that wrap raw OpenGL shader/program APIs.
//!
//! The `Program` type owns a linked shader program, while `Shader` compiles
//! individual vertex/fragment stages from source strings. This keeps the
//! OpenGL-specific glue in one place so the rest of the app can focus on data
//! setup and drawing.

use gl;
use std;
use std::ffi::{CStr, CString};

/// RAII wrapper around an OpenGL program object.
///
/// Instances are created by linking an arbitrary list of compiled [`Shader`]s.
/// When dropped, the underlying GL program is deleted automatically.
pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    /// Links the provided shaders into a program object.
    ///
    /// # Errors
    ///
    /// Returns a descriptive error log if linking fails (for example, when
    /// shader interfaces mismatch).
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()) };
        }

        unsafe { gl::LinkProgram(program_id) };

        // Error handling here
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()) };
        }

        Ok(Program { id: program_id })
    }

    /// Exposes the raw OpenGL program identifier.
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    /// Binds this program for subsequent draw calls.
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

/// RAII wrapper around a compiled shader stage.
pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    /// Compiles GLSL source of the given `kind` (vertex/fragment).
    ///
    /// # Errors
    ///
    /// Returns the shader compiler log if compilation fails.
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    /// Convenience wrapper for vertex shaders.
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    /// Convenience wrapper for fragment shaders.
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    /// Returns the raw OpenGL shader identifier.
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

/// Compiles a shader of `kind` from a C-compatible source string.
fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id)
    };

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    };

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            )
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

/// Allocates a mutable `CString` filled with ASCII spaces to receive driver logs.
fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Buffer to write the error
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));

    unsafe { CString::from_vec_unchecked(buffer) }
}
