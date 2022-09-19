#version 450

layout(push_constant) uniform constants {
    vec4 color;
} pc;

layout(location=1) in vec2 position;

void main() {
    gl_Position = vec4(position, 0., 1.);
}