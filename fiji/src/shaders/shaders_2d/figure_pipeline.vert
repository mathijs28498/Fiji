#version 450

layout(push_constant) uniform Constants {
    uvec2 resolution;
    vec2 position;
    vec4 borderColor;
    vec2 size;
    vec2 cameraPos;
    uint borderWidth; 
} pc;

layout(location=0) in vec2 position;

vec2 worldToScreen(vec2 worldPos);

void main() {
    vec2 worldPos = (pc.cameraPos + pc.position + position * pc.size);
    vec2 pos = worldToScreen(worldPos);
    gl_Position = vec4(pos, 0., 1.);
}

vec2 worldToScreen(vec2 worldPos) {
    return worldPos / pc.resolution * 2. - 1.;
}