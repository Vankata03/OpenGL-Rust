//! Entry point for the example application.
//!
//! The executable bootstraps SDL2, requests an OpenGL 3.3 core context, and
//! draws a single RGB triangle every frame. The data flows through the modern
//! programmable pipeline: vertex data in a VBO → vertex shader → rasterizer →
//! fragment shader → default framebuffer.

extern crate gl;
extern crate sdl2;

pub mod render_gl;

use std::ffi::CString;

fn main() {
    // Initialize SDL2 and request a core OpenGL profile.
    let sdl = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl.video().expect("Failed to initialize video subsystem");
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    // Create the window that will own the OpenGL framebuffer.
    let window = video_subsystem
        .window("OpenGL Window", 1200, 800)
        .opengl()
        .resizable()
        .build()
        .expect("Failed to create window");

    // SDL event pump used to process OS window/input callbacks.
    let mut event_pump = sdl.event_pump().expect("Failed to get event pump");

    // Load OpenGL function pointers resolved through SDL.
    let _gl_context = window
        .gl_create_context()
        .expect("Failed to create GL Context");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Configure the default viewport and clear color.
    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // Compile the programmable stages that make up the pipeline.
    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    shader_program.set_used();

    // Interleaved vertex buffer: XYZ position followed by RGB color per vertex.
    let vertices: Vec<f32> = vec![
        // Position         // Colors
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        // Upload vertex data once (STATIC_DRAW) because the triangle never changes.
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // Vertex Array Object remembers how to interpret the buffer for each attribute.
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Attribute 0: clip-space position (first 3 floats).
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );

        // Attribute 1: vertex color (3 floats after the position).
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Main program loop
    'main: loop {
        // Poll window/input events; exit when the OS asks us to quit.
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::AppLowMemory { .. } => break 'main,
                _ => {}
            }
        }

        // Clear the framebuffer and render the triangle.
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw the triangle
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
}
