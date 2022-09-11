#version 450

layout(push_constant) uniform constants {
    uvec2 resolution;
    vec2 position;
    vec4 color;
    float radius;
} pc;

layout(location=1) in vec2 position;

vec2 worldToScreen(vec2 worldPos);

void main() {
    gl_Position = vec4(worldToScreen(position), 0., 1.);
}

vec2 worldToScreen(vec2 worldPos) {
    return (pc.position + position * pc.radius) / pc.resolution * 2. - 1.;
}