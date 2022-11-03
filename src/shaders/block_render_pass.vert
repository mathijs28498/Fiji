#version 450

layout(push_constant) uniform constants {
    vec4 color;
    mat4 world;
    mat4 view;
    mat4 proj;
    uvec2 resolution;
} pc;

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

layout (location = 0) out vec3 fPosition;
layout (location = 1) out vec3 fNormal;

void main() {
    vec4 worldPos = pc.world * vec4(position, 1.);
    fPosition = worldPos.xyz;
    fNormal = mat3(transpose(inverse(pc.world))) * normal;
    gl_Position = pc.proj * pc.view * worldPos;
}