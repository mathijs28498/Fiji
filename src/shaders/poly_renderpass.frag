#version 450

layout(push_constant) uniform constants {
    vec2 position;
    vec2 size;
    vec3 color;
    uvec2 resolution;
} pc;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(pc.color, 1.);
}