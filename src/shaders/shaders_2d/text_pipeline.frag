// #version 450

// layout(location = 0) in vec2 fUvCoord;

// layout(location=0) out vec4 f_color;

// layout (set = 0, binding = 0) uniform usampler2D charTex;

// void main() {
//     uvec2 test = pc.resolution;
//     vec2 test2 = fUvCoord;
//     vec4 test3 = texture(charTex, fUvCoord);

//     f_color = vec4(vec3(test3.r) / 255.0, 1.);
// }

#version 450

layout(push_constant) uniform Constants {
    uvec2 resolution;
    vec2 position;
    vec4 color;
} pc;

layout (location = 0) in vec2 fUvCoord;

layout (location = 0) out vec4 f_color;

layout (set = 0, binding = 0) uniform usampler2D charTex;

void main() {
    uint sampled = texture(charTex, fUvCoord).r;

    // TODO: Check if it is better with alpha
    float alpha = 0.;
    if (sampled > 0) {
        alpha = 1.;
    }

    vec4 charValue = vec4(vec3(sampled / 255.), alpha);
    f_color = pc.color * charValue;
}