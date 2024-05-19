#extension GL_EXT_samplerless_texture_functions : enable

#define MAX_RAY_STEPS 1000

struct RayCastResult {
    vec3 normal;
    uint hit_block_id;
};

RayCastResult ray_cast(vec3 origin, vec3 direction, utexture3D voxel_texture) {
    ivec3 grid_size = textureSize(voxel_texture, 0);

    origin = origin * grid_size + grid_size / 2;

    ivec3 mapPos = ivec3(origin);
    vec3 deltaDist = abs(vec3(length(direction)) / direction);
    ivec3 rayStep = ivec3(sign(direction));

    vec3 sideDist = (sign(direction) * (mapPos - origin) + (sign(direction) * 0.5) + 0.5) * deltaDist;
    vec3 dis_step = abs(rayStep);

    vec3 ray_normal_dir = ceil(-direction) - direction;

    vec3 normal;
    uint hit;
    uint ray_steps;

    for (int i = 0; i < MAX_RAY_STEPS; i++) {
        uint res = texelFetch(voxel_texture, mapPos, 0).r;
        if (res != 0) {
            hit = res;
            break;
        }

        if (sideDist.x < sideDist.y) {
            if (sideDist.x < sideDist.z) {
                sideDist.x += deltaDist.x;
                mapPos.x += rayStep.x;
                normal = vec3(ray_normal_dir.x, 0.0, 0.0);
            } else {
                sideDist.z += deltaDist.z;
                mapPos.z += rayStep.z;
                normal = vec3(0.0, 0.0, ray_normal_dir.z);
            }
        } else {
            if (sideDist.y < sideDist.z) {
                sideDist.y += deltaDist.y;
                mapPos.y += rayStep.y;
                normal = vec3(0.0, ray_normal_dir.y, 0.0);
            } else {
                sideDist.z += deltaDist.z;
                mapPos.z += rayStep.z;
                normal = vec3(0.0, 0.0, ray_normal_dir.z);
            }
        }
        // if (clamp(mapPos, ivec3(0), grid_size) != mapPos) {
        //     break;
        // }
    }

    RayCastResult res = RayCastResult(normal, hit);

    return res;
}

vec3 rayCubeIntersection(vec3 rayOrigin, vec3 rayDirection, vec3 cubeMin, vec3 cubeMax) {
    if (rayOrigin.x >= cubeMin.x && rayOrigin.x <= cubeMax.x &&
            rayOrigin.y >= cubeMin.y && rayOrigin.y <= cubeMax.y &&
            rayOrigin.z >= cubeMin.z && rayOrigin.z <= cubeMax.z) {
        return rayOrigin;
    }

    float tmin = (cubeMin.x - rayOrigin.x) / rayDirection.x;
    float tmax = (cubeMax.x - rayOrigin.x) / rayDirection.x;

    if (tmin > tmax) {
        float temp = tmin;
        tmin = tmax;
        tmax = temp;
    }

    float tymin = (cubeMin.y - rayOrigin.y) / rayDirection.y;
    float tymax = (cubeMax.y - rayOrigin.y) / rayDirection.y;

    if (tymin > tymax) {
        float temp = tymin;
        tymin = tymax;
        tymax = temp;
    }

    if (tmin > tymax || tymin > tmax) {
        return rayOrigin;
    }

    if (tymin > tmin) {
        tmin = tymin;
    }

    if (tymax < tmax) {
        tmax = tymax;
    }

    float tzmin = (cubeMin.z - rayOrigin.z) / rayDirection.z;
    float tzmax = (cubeMax.z - rayOrigin.z) / rayDirection.z;

    if (tzmin > tzmax) {
        float temp = tzmin;
        tzmin = tzmax;
        tzmax = temp;
    }

    if (tmin > tzmax || tzmin > tmax) {
        return vec3(100000.0);
    }

    if (tzmin > tmin) {
        tmin = tzmin;
    }

    if (tzmax < tmax) {
        tmax = tzmax;
    }

    return rayOrigin + rayDirection * tmax;
}
