#version 460
#include "./ray_cast.glsl"

layout(location = 0) in vec3 VertexPos;
layout(location = 1) in vec3 InstancePos;

layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform Camera {
    mat4 proj;
    vec3 pos;
} camera;

layout(set = 1, binding = 0) uniform utexture3D voxel_texture;


void main() {
    vec3 camPos = camera.pos - InstancePos;
    vec3 rayDir = normalize(VertexPos - camPos);
    vec3 rayPos = rayCubeIntersection(camPos, -rayDir, vec3(-0.5), vec3(0.5));

    RayCastResult res = ray_cast(rayPos, rayDir, voxel_texture);


    if (res.hit_block_id == 0) {
        f_color = vec4(0.0);
        return;
    }

    vec3 light_dir = vec3(0.1, -1.0, 0.1);
    float dot_product = dot(light_dir, res.normal) / 2.0 + 0.5;

    float light_val = max(dot_product, 0.3);

    f_color = vec4(vec3(dot_product), 1.0);
}
