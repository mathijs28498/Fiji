#version 450

layout(push_constant) uniform Constants {
    uvec2 resolution;
    vec2 position;
    vec4 color;
    vec4 borderColor;
    uint borderWidth;
    float radius;
} pc;

layout(location=0) out vec4 f_color;

void main() {
    float dist_center = distance(gl_FragCoord.xy, pc.position);
    if (dist_center > pc.radius + pc.borderWidth) {
        discard;
    } else if (dist_center > pc.radius) {
        f_color = pc.borderColor;
    } else {
        f_color = pc.color;
    }
}