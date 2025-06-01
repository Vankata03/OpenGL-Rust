extern crate gl;
extern crate sdl2;

pub mod render_gl;

fn main() {
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

    use std::ffi::CString;
    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    shader_program.set_used();

    // VBO = Vertex Buffer Object
    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // VAO = Vertex Array Object
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
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

        // Draw the triangle
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
}
