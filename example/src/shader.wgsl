struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;



struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv_cords: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv_cords: vec2<f32>,
    @location(1) normal: vec3<f32>,
}


@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out = VertexOutput();
    let offset = vec3(-0.5, -0.5, 0.0);
    out.clip_position = camera.view_proj * vec4((vertex.position) * vec3(0.5), 1.0);
    out.uv_cords = vertex.uv_cords;
    out.normal = vertex.normal;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.normal, 1.0);
}
