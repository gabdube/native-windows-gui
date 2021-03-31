#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 inFragPos;
layout (location = 1) in vec3 inNorm;

layout (set=1, binding = 0) uniform LIGHT 
{
    vec4 pos;
    vec4 color;
    vec4 viewPos;
} light;

layout (set=0, binding = 1) uniform MATERIAL 
{
    vec4 color;
    vec4 spec;   // [0]: spec strength / [1] spec shininess
} mat;


layout (location = 0) out vec4 outFragColor;

void main() 
{
    // ambient
    float ambientStrength = light.color.a;
    vec3 ambient = ambientStrength * light.color.rgb;

    // diffuse 
    vec3 norm = normalize(inNorm);
    vec3 lightDir = normalize(light.pos.xyz - inFragPos);
    vec3 diffuse = max(dot(norm, lightDir), 0.0) * light.color.rgb;

    // specular
    float specularStrength = mat.spec[0];
    vec3 viewDir = normalize(light.viewPos.xyz - inFragPos);
    vec3 reflectDir = reflect(-lightDir, norm);  
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), mat.spec[1]);
    vec3 specular = specularStrength * spec * light.color.rgb;

    vec3 result = (ambient + diffuse + specular) * mat.color.rgb;
    outFragColor = vec4(result, 1.0);
}