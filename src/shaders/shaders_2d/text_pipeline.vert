#version 450

layout(push_constant) uniform Constants {
    uvec2 resolution;
    vec2 position;
    vec4 color;
} pc;

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 uvCoord;

layout(location = 0) out vec2 fUvCoord;

vec2 worldToScreen(vec2 worldPos);

void main() {
    vec2 pos = worldToScreen(position + pc.position);
    gl_Position = vec4(pos, 0., 1.);
    fUvCoord = uvCoord;
}

vec2 worldToScreen(vec2 worldPos) {
    return worldPos / pc.resolution * 2. - 1.;
}

// #version 450

// layout (push_constant) uniform Constants {
//     uvec2 resolution;
// } pc;

// layout (location = 0) in vec2 position;

// vec2 worldToScreen(vec2 worldPos);

// void main() {
//     gl_Position = vec4(worldToScreen(position), 0., 1.);
//     gl_Position = vec4(position, 0., 1.);
// }

// vec2 worldToScreen(vec2 worldPos) {
//     return worldPos / pc.resolution * 2. - 1.;
// }

// #version 450

// layout(location = 0) in vec2 position;
// layout(location = 1) in vec2 uvCoords;

// layout (location = 0) out vec2 fUvCoords;

// void main() {
//     gl_Position = vec4(position, 0., 1.);
//     fUvCoords = uvCoords;
// }