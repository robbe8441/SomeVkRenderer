struct CameraUniform {
    view_proj: mat4x4<f32>,
    camera_pos : vec4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
}



@vertex
fn vs_main(in : VertexInput) -> VertexOutput {
    var out : VertexOutput;
    let pos = (in.position + vec3(-0.5, -0.5, 0.0)) * vec3(2.0);


    out.clip_position = vec4(pos, 1.0);
    out.uv = in.uv;
    return out;
}


struct Uniforms {
    time : f32,
    width : f32,
    height : f32,
    mode : i32,
};
@group(1) @binding(0)
    var<uniform> uniforms: Uniforms;
    
@group(1) @binding(1)
    var voxel_data: texture_3d<u32>;
@group(1) @binding(2)
    var<storage, read> color_buffer: array<vec4<f32>>;



fn getVoxel(c : vec3<i32>) -> Voxel {
        let pos = vec3<i32>(c.x, -c.y, c.z / 2);
        let val = textureLoad(voxel_data, pos, 0).r;

        var voxel = Voxel();

        //voxel.color = color_buffer[val];
        voxel.color = vec4(0.0);

        if val < 100 { voxel.is_empty = true; voxel.color = vec4(1.0); }

	return voxel;
}

struct RaycastResult {
   normal : vec3<f32>,
   position : vec3<f32>,
   color : vec4<f32>,
   distance : f32,
   hit : bool,
   stepps : i32,
}

struct Voxel {
    is_empty : bool,
    color : vec4<f32>,
}


fn raycast(origin : vec3<f32>, direction : vec3<f32>) -> RaycastResult  {

    let MAX_RAY_STEPPS : i32 = 500;

    var mapPos = vec3<i32>(floor(origin));
    var deltaDist = abs(vec3(length(direction)) / direction);
    let rayStep = vec3<i32>(sign(direction));
    var sideDist = (sign(direction) * (vec3<f32>(mapPos) - origin) + (sign(direction) * 0.5) + 0.5) * deltaDist; 
    let dis_step = vec3<f32>(abs(rayStep));

    let ray_normal_dir = ceil(-direction) - direction;

    var normal = vec3<f32>(0.0);
    var dis = 0.0;
    var hit = false;
    var color = vec4(0.5, 1.0, 1.0, 0.0);

    var stepps = 0;

    for (var i=0; i<MAX_RAY_STEPPS; i+=1) {
        stepps += 1;

        if (sideDist.x < sideDist.y) {
            if (sideDist.x < sideDist.z) {
                    sideDist.x += deltaDist.x;
                    mapPos.x += rayStep.x;
                    normal = vec3(ray_normal_dir.x, 0.0, 0.0);
                    dis += dis_step.x;
                } else {
                    sideDist.z += deltaDist.z;
                    mapPos.z += rayStep.z;
                    normal = vec3(0.0, 0.0, ray_normal_dir.z);
                    dis += dis_step.z;
                }

            } else {
                if (sideDist.y < sideDist.z) {
                        sideDist.y += deltaDist.y;
                        mapPos.y += rayStep.y;
                        normal = vec3(0.0, ray_normal_dir.y, 0.0);
                        dis += dis_step.y;
                } else {
                        sideDist.z += deltaDist.z;
                        mapPos.z += rayStep.z;
                        normal = vec3(0.0, 0.0, ray_normal_dir.z);
                        dis += dis_step.z;
                }
            }
            let voxel = getVoxel(mapPos);
            if !voxel.is_empty {hit = true; color = voxel.color; break;}
        }

        var res = RaycastResult();
        res.normal = normal;
        res.distance = dis;
        res.position = vec3<f32>(mapPos) + normal;
        res.hit = hit;
        res.color = color;
        res.stepps = stepps;

        return res;
}







@fragment
fn fs_main(in : VertexOutput) -> @location(0) vec4<f32> {

    let screenPos = in.uv;

    var cameraDir = vec3(0.0, 0.0, -1.0);
    let cameraPlaneU = vec3(1.0, 0.0, 0.0);
    let cameraPlaneV = vec3(0.0, 1.0, 0.0) * uniforms.height / uniforms.width;

    let rayDir = (vec4(cameraDir + screenPos.x * cameraPlaneU + screenPos.y * cameraPlaneV, 1.0) * camera.view_proj).xyz;
    let rayPos = camera.camera_pos.xyz;

    let RayRes = raycast(rayPos, rayDir);

    if (!RayRes.hit) {
        return vec4(0.0);
    }

    switch uniforms.mode {
        case 1: {
            return vec4(vec3(f32(RayRes.stepps) / 1000.0), 1.0);
        }
        default: {
            return vec4(
                vec3(dot(RayRes.normal.rgb, vec3(1.0, 1.0, 0.0)) / 10.0),
                1.0);
            }
        }

}




