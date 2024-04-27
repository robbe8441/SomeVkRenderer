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
struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv_cords: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) position: vec3<f32>,
}

struct TestUniform {
    time: f32
}
@group(1) @binding(0)
var<uniform> test_uniforms: TestUniform;


@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out = VertexOutput();
    let offset = vec3(-0.5, -0.5, 0.0);
    
    out.clip_position = camera.view_proj * vec4((vertex.position) * vec3(0.5), 1.0);
    out.uv_cords = vertex.uv_cords;
    out.normal = vertex.normal;
    out.position = vertex.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let light_dir = normalize(vec3(1.0, 0.5, 1.0));

    let dot = dot(in.normal, light_dir) + 0.5;

    //let base_color = in.position.xyz / 100.0 * sin(test_uniforms.time / dot);

    return vec4(vec3(dot / 2.0), 1.0);
}
