#version 460

#define MAX_RAY_STEPS 300

layout(location = 1) in vec3 VertexPos;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform Camera {
    mat4 proj;
    vec3 pos;
} camera;

// layout(set = 1, binding = 0) uniform utexture3D tex;

float SphereSDF(vec3 pos, float r) {
    return length(pos) - r;
}

vec3 ray_cast() {
    vec3 rayDir = normalize(VertexPos - camera.pos);
    vec3 rayPos = camera.pos;  // TODO: FIX THIS!!

    float current_dis = 0.0;

    int steps;

    while (current_dis < 10.0) {
        steps += 1;

        float step_size = SphereSDF(rayPos + rayDir * current_dis, 0.1);

        if (step_size < 0.01) {
            return vec3(1.0 - step_size);
        }

        current_dis += step_size;
    }


    return vec3(steps / 10.0);
}

void main() {
    vec3 dis = ray_cast();

    f_color = vec4(dis, 1.0);
}




