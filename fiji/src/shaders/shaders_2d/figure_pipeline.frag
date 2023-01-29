#version 450

layout(push_constant) uniform Constants {
    uvec2 resolution;
    vec2 position;
    vec4 borderColor;
    vec2 size;
    vec2 cameraPos;
    uint borderWidth; 
} pc;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(1.);
}