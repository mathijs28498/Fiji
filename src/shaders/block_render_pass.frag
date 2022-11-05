#version 450

layout(push_constant) uniform Constants {
    vec4 color;
    mat4 world;
    mat4 view;
    mat4 proj;
    uvec2 resolution;
} pc;

layout (location = 0) in vec3 fPosition;
layout (location = 1) in vec3 fNormal;

layout (location = 0) out vec4 f_color;

const vec3 lightPos = vec3(5., 5., -5);
const float ambientStrength = 0.3;
const float diffuseStrength = 1. - ambientStrength;

void main() {
    vec3 dir = normalize(lightPos - fPosition);
    float lightStrength = max(dot(normalize(fNormal), dir), 0.0);
    vec3 colorWithoutTransparency = pc.color.xyz;

    f_color = vec4(colorWithoutTransparency * lightStrength * diffuseStrength + colorWithoutTransparency * ambientStrength, 1);// pc.color.w);
}