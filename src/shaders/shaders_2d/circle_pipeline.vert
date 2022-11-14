#version 450

layout(push_constant) uniform Constants {
    uvec2 resolution;
    vec2 position;
    vec4 color;
    vec4 borderColor;
    uint borderWidth;
    float radius;
} pc;

layout (location = 0) in vec2 position;

vec2 worldToScreen(vec2 worldPos);

void main() {
    gl_Position = vec4(worldToScreen(position), 0., 1.);
}

vec2 worldToScreen(vec2 worldPos) {
    return (pc.position + position * (pc.radius + pc.borderWidth)) / pc.resolution * 2. - 1.;
}