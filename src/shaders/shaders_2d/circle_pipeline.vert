#version 450

layout(push_constant) uniform Constants {
    uvec2 resolution;
    vec2 position;
    vec4 color;
    vec4 borderColor;
    uint borderWidth;
    float radius;
    vec2 cameraPos;
} pc;

layout (location = 0) in vec2 position;

vec2 worldToScreen(vec2 worldPos);

void main() {
    vec2 worldPos = pc.position + pc.cameraPos + position * (pc.radius + pc.borderWidth);
    gl_Position = vec4(worldToScreen(worldPos), 0., 1.);
}

vec2 worldToScreen(vec2 worldPos) {
    return worldPos / pc.resolution * 2. - 1.;
}