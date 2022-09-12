#version 450

layout(push_constant) uniform constants {
    uvec2 resolution;
    vec2 position;
    vec4 color;
    vec4 borderColor;
    vec2 size;
    float borderWidth;    
} pc;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = pc.color;
}