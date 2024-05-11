#version 460
#extension GL_EXT_samplerless_texture_functions : enable

#define MAX_RAY_STEPS 300

layout(location = 1) in vec3 VertexPos;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform Camera {
    mat4 proj;
    vec3 pos;
} camera;

layout(set = 1, binding = 0) uniform utexture3D tex;

uint GetVoxel(vec3 pos) {
    ivec3 size = textureSize(tex,0) + 1;
    return texelFetch(tex, ivec3((pos + 0.5) * size - 1.0), 0).r;
}

vec3 ray_cast() {
    vec3 rayDir = normalize(VertexPos - camera.pos);
    vec3 rayPos = camera.pos;  // TODO: FIX THIS!!

    float current_dis = 0.0;

    int steps;

    while (current_dis < 10.0) {
        steps += 1;

        uint result = GetVoxel(rayPos + rayDir * current_dis);

        if (result != 0) {
            return vec3(result / 255 - current_dis / 6.0);
        }

        current_dis += 0.01;
    }


    return vec3(0.1);
}

void main() {
    vec3 dis = ray_cast();

    f_color = vec4(dis, 1.0);
}




