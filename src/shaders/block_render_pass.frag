#version 450

layout(push_constant) uniform Constants {
    vec4 color;
    mat4 world;
    mat4 view;
    mat4 proj;
    vec3 cameraPos;
    uvec2 resolution;
} pc;

layout (location = 0) in vec3 fPosition;
layout (location = 1) in vec3 fNormal;

layout (location = 0) out vec4 f_color;

const vec3 lightPos = vec3(5., -5., -5.);
const vec3 lightColor = vec3(1., 1., 1.);

const float ambientStrength = 0.2;
const float specularStrength = 1.0;

void main() {
    // Calculate ambient light
    vec3 ambient = ambientStrength * lightColor;
    
    // Calculate diffuse light
    vec3 lightDir = normalize(lightPos - fPosition);
    float diff = max(dot(normalize(fNormal), lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    // Calculate specular light
    vec3 viewDir = normalize(pc.cameraPos - fPosition);
    vec3 reflectDir = reflect(-lightDir, fNormal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 256);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * pc.color.rgb;
    // vec3 result = (ambient + diffuse) * pc.color.rgb;
    f_color = vec4(result, pc.color.w);
}