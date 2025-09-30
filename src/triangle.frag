#version 330 core

// Fragment shader: receives interpolated RGB colors and writes them to the
// default framebuffer with an opaque alpha channel.
in VS_OUTPUT {
    vec3 Color;
} IN;

out vec4 Color;

void main() 
{
    Color = vec4(IN.Color, 1.0f);
}