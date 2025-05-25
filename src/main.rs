extern crate sdl2;
extern crate gl;

fn main() {

    // Initialize sdl and create the windows with OpenGL
    let sdl = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl.video().expect("Failed to initialize video subsystem");
    let window = video_subsystem
        .window("OpenGL Window", 1200, 800)
        .opengl()
        .resizable()
        .build()
        .expect("Failed to create window");


    // Get the event pump and gl context
    let mut event_pump = sdl.event_pump().expect("Failed to get event pump");
    let gl_context = window.gl_create_context().expect("Failed to create GL Context");

    // Main program loop
    'main: loop {
        for event in event_pump.poll_iter() {
            
            // User input
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // Render here
    }

}
