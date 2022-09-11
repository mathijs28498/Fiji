#version 450

layout(push_constant) uniform constants {
    uvec2 resolution;
    vec2 position;
    vec4 color;
    float radius;
} pc;

layout(location=0) out vec4 f_color;

void main() {
    float dist_center = distance(gl_FragCoord.xy, pc.position);
    if (dist_center > pc.radius) {
        discard;
    }

    f_color = pc.color;
}