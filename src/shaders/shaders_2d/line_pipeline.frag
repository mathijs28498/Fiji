#version 450

layout(push_constant) uniform Constants {
    vec4 color;
} pc;

layout(location=0) out vec4 f_color;

void main() {
    f_color = pc.color;
}