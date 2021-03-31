#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 inPos;
layout (location = 1) in vec3 inNorm;

layout (location = 0) out vec3 outFragPos;
layout (location = 1) out vec3 outNorm;

layout (set=0, binding = 0) uniform UBO 
{
    mat4 mvp;
    mat4 model;
    mat4 normal;
};


void main() 
{
    outFragPos = vec3(model * vec4(inPos, 1.0));
    outNorm = mat3(normal) * inNorm;

    gl_Position = mvp * vec4(inPos, 1.0);
}
