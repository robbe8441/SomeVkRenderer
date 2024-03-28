struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv_cords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv_cords: vec2<f32>,
}


@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out = VertexOutput();
    let offset = vec3(-0.5, -0.5, 0.0);
    out.clip_position = vec4((vertex.position + offset) * vec3(2.0), 1.0);
    out.uv_cords = vertex.uv_cords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.uv_cords.xy, 1.0 , 1.0);
}





