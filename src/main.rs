extern crate gl;
extern crate sdl2;

// Note: Just used for debugging
use std::{env, ffi::CString};

fn main() {
    // Back Trace for debuging
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    // Initialize sdl and do some setup work
    let sdl = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl.video().expect("Failed to initialize video subsystem");
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    // Create the windows with OpenGL
    let window = video_subsystem
        .window("OpenGL Window", 1200, 800)
        .opengl()
        .resizable()
        .build()
        .expect("Failed to create window");

    // Get the event pump
    let mut event_pump = sdl.event_pump().expect("Failed to get event pump");

    // Load OpenGL function pointers
    let _gl_context = window
        .gl_create_context()
        .expect("Failed to create GL Context");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Set up viewport
    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // Main program loop
    'main: loop {
        // Listen for events/user input
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::AppLowMemory { .. } => break 'main,
                _ => {}
            }
        }

        // Render here
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }

    struct Shader {
        id: gl::types::GLuint,
    }

    impl Shader {
        fn from_source(source: &std::ffi::CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
            let id = shader_from_source(source, kind)?;
            Ok(Shader { id })
        }

        fn from_vert_source(source: &std::ffi::CStr) -> Result<Shader, String> {
            Shader::from_source(source, gl::VERTEX_SHADER)
        }

        fn from_frag_source(source: &std::ffi::CStr) -> Result<Shader, String> {
            Shader::from_source(source, gl::FRAGMENT_SHADER)
        }
    }

    impl Drop for Shader {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteShader(self.id);
            }
        }
    }

    fn shader_from_source(
        source: &std::ffi::CStr,
        kind: gl::types::GLuint,
    ) -> Result<gl::types::GLuint, String> {
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

    fn create_whitespace_cstring_with_len(len: usize) -> std::ffi::CString {
        // Buffer to write the error
        let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
        buffer.extend([b' '].iter().cycle().take(len));

        unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
    }
}
