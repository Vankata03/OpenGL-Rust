#version 330 core

// Vertex shader: forwards clip-space positions to the rasterizer and passes
// per-vertex RGB colors to the fragment stage.
layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    gl_Position = vec4(Position, 1.0);
    OUT.Color = Color;
}