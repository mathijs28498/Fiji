#version 450

layout(push_constant) uniform constants {
    vec4 color;
    mat4 world;
    mat4 view;
    mat4 proj;
    uvec2 resolution;
} pc;

layout (location = 0) out vec4 f_color;

void main() {
    f_color = pc.color;
}