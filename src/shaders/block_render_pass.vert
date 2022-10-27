#version 450

layout(push_constant) uniform constants {
    vec4 position;
    vec4 size;
    vec4 color;
    mat4 proj;
    uvec2 resolution;
} pc;

layout (location = 0) in vec3 position;

void main() {
    // gl_Position = vec4(position, 1.) * pc.proj;
    gl_Position = vec4(position, 1.) * pc.proj;// + vec4(0.2, 0.2, 0., 1.);
}