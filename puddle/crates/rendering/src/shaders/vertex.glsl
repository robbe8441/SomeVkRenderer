#version 450

layout(location = 0) in vec3 position;

layout(location = 0) out vec3 v_normal;
layout(location = 1) out vec3 VertexPos;

layout(set = 0, binding = 0) uniform Camera {
    mat4 proj;
    vec3 pos;
} uniforms;

void main() {
    VertexPos = position.xyz;
    gl_Position = uniforms.proj * vec4(position, 1.0);
}
