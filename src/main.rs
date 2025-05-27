extern crate gl;
extern crate sdl2;

// Note: Just used for debugging
use std::env;

fn main() {
    // Back Trace for debuging
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    // Initialize sdl and create the windows with OpenGL
    let sdl = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl.video().expect("Failed to initialize video subsystem");
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

    unsafe {
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
}
