#version 450

layout(push_constant) uniform constants {
    vec2 position;
    vec2 size;
    vec3 color;
    uvec2 resolution;
} pc;

layout(location=0) in vec2 position;

void main() {
    vec2 pos = (pc.position + position * pc.size) / pc.resolution * 2. - 1.;
    gl_Position = vec4(pos, 0., 1.);
}
