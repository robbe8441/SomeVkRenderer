
struct CameraUniform {
    view_proj: mat4x4<f32>,
    camera_eye: vec4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv_cords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) uv_cords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position * vec3(1.0,1.0,-1.0) + camera.camera_eye.xyz, 1.0);
    out.uv_cords = model.uv_cords;

    return out;
}



@group(1) @binding(0)
    var sky_texture: texture_2d<f32>;
@group(1) @binding(1)
    var texture_sampler: sampler;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(sky_texture, texture_sampler, in.uv_cords);
}









