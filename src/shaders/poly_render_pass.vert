#version 450

layout(push_constant) uniform constants {
    uvec2 resolution;
    vec2 position;
    vec4 color;
    vec2 size;
} pc;

layout(location=0) in vec2 position;

vec2 worldToScreen(vec2 worldPos);

void main() {
    vec2 pos = worldToScreen(position);
    gl_Position = vec4(pos, 0., 1.);
}

vec2 worldToScreen(vec2 worldPos) {
    return (pc.position + position * pc.size) / pc.resolution * 2. - 1.;
}