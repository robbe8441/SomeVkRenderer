#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 instance_position;

layout(location = 0) out vec3 VertexPos;
layout(location = 1) out vec3 InstancePos;

layout(set = 0, binding = 0) uniform Camera {
    mat4 proj;
    vec3 pos;
} uniforms;

void main() {
    VertexPos = position;
    InstancePos = instance_position;
    gl_Position = uniforms.proj * vec4(position + instance_position, 1.0);
}
