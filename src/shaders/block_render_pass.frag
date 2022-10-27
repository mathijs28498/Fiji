#version 450

layout(push_constant) uniform constants {
    vec4 color;
    vec4 position;
    vec4 size;
    mat4 proj;
    uvec2 resolution;
} pc;

layout (location = 0) out vec4 f_color;

void main() {
    f_color = pc.color;
}