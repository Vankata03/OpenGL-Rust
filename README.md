# OpenGL-Rust

This project is a basic OpenGL learning playground written in Rust. It uses the [SDL2](https://crates.io/crates/sdl2) and [gl](https://crates.io/crates/gl) crates to create a window and render with OpenGL. The goal is to follow and experiment with various OpenGL tutorials, translating them into Rust for learning and practice.

## Features

- SDL2 window and event handling
- OpenGL context creation (core profile 3.3)
- Basic rendering loop that draws a single RGB triangle
- Reusable shader/program helpers in `src/render_gl.rs`

## Getting Started

To get started with this project, clone the repository:

```sh
git clone https://github.com/Vankata03/OpenGL-Rust.git
```

### Prerequisites

- Rust (latest stable, [install here](https://rustup.rs/))
- SDL2 library installed on your system
  - On macOS: `brew install sdl2`
  - On Linux: `sudo apt-get install libsdl2-dev`
  - On Windows: [SDL2 downloads](https://www.libsdl.org/download-2.0.php)

### Build and Run

```sh
cargo run
```

## How the pieces fit together

- `src/main.rs` bootstraps SDL2, requests an OpenGL 3.3 core context, and drives the event/render loop.
- `src/render_gl.rs` wraps low-level OpenGL calls for compiling shaders and linking them into a program.
- `src/triangle.vert` & `src/triangle.frag` define a modern programmable pipeline pair that transforms vertex positions and colors into pixels on screen.
- Vertex data lives in an interleaved VBO (positions + colors) and is referenced via a VAO to keep attribute layout tidy.

## OpenGL pipeline overview

```text
CPU vertex data (Vec<f32>)
  │ upload via glBufferData
  ▼
Vertex Buffer Object (VBO)
  │ attribute views recorded in VAO
  ▼
Vertex Shader (triangle.vert)
  │ emits clip-space position + color
  ▼
Rasterizer & Interpolation
  │ generate fragments from the triangle
  ▼
Fragment Shader (triangle.frag)
  │ outputs final RGBA
  ▼
Default Framebuffer → SDL2 window swap chain
```

## References & Tutorials

- [Rust Programming Language](https://www.rust-lang.org/)
- [Main Tutorial - OpenGL in Rust](https://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html)
- [LearnOpenGL](https://learnopengl.com/)
- [OpenGL Tutorial](https://www.opengl-tutorial.org/)
- [Rust-SDL2 Examples](https://github.com/Rust-SDL2/rust-sdl2)

## License

MIT
