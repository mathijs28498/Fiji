#version 450

layout(push_constant) uniform constants {
    vec4 color;
    mat4 world;
    mat4 view;
    mat4 proj;
    uvec2 resolution;
} pc;

layout (location = 0) in vec3 position;

void main() {
    // gl_Position = vec4(position, 1.) * pc.model * pc.world * pc.proj;
    // gl_Position = vec4(position, 1.) * pc.proj;
    gl_Position = pc.proj * pc.view * pc.world * vec4(position, 1.);
    // gl_Position = pc.proj * pc.world * pc.model * vec4(position, 1.);
    // gl_Position = pc.proj * vec4(position, 1.);
}