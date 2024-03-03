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
};
@group(1) @binding(0)
    var<uniform> uniforms: Uniforms;
    
@group(1) @binding(1)
    var voxel_data: texture_3d<u32>;






fn sdSphere(p : vec3<f32>, d : f32) -> f32 { 
    return length(p) - d;
} 

fn sdBox( p : vec3<f32>, b : vec3<f32> ) -> f32 {
  let d = abs(p) - b;
  return min(max(d.x ,max(d.y, d.z)),0.0) +
         length(max(d,vec3(0.0)));
}


fn getVoxel(c : vec3<i32>) -> bool {
        let val = textureLoad(voxel_data, c, 0).r;

        if sdSphere(vec3<f32>(c) + 0.5 - camera.camera_pos.xyz, 30.0) < 0.0 {
            return false;
        }

	return val > 0;
}

struct RaycastResult {
   normal : vec3<f32>,
   position : vec3<f32>,
   distance : f32,
   hit : bool,
}


fn raycast(origin : vec3<f32>, direction : vec3<f32>) -> RaycastResult  {

    let MAX_RAY_STEPPS : i32 = 1000;

    var mapPos = vec3<i32>(floor(origin));
    var deltaDist = abs(vec3(length(direction)) / direction);
    let rayStep = vec3<i32>(sign(direction));
    var sideDist = (sign(direction) * (vec3<f32>(mapPos) - origin) + (sign(direction) * 0.5) + 0.5) * deltaDist; 
    let dis_step = vec3<f32>(abs(rayStep));

    let ray_normal_dir = ceil(-direction) - direction;

    var normal = vec3<f32>(0.0);
    var dis = 0.0;
    var hit = false;


    for (var i=0; i<MAX_RAY_STEPPS; i+=1) {
        if getVoxel(mapPos) {hit = true; break;}

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
        }

        var res = RaycastResult();
        res.normal = normal;
        res.distance = dis;
        res.position = vec3<f32>(mapPos) + normal;
        res.hit = hit;

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

    var color = vec3(0.0);
    var dis = 0.0;

    let RayRes = raycast(rayPos, rayDir);
    dis = RayRes.distance;


    if (!RayRes.hit) {
        return vec4(0.0);
    }

    return vec4(vec3(dis / 100.0), 1.0);
    //return vec4(color, 1.0);
}




