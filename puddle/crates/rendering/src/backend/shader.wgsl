struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv_cords: vec2<f32>,
};


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>
}


@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out = VertexOutput();
    out.clip_position = vec4(vertex.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(1.0, 0.0, 0.0, 1.0);
}





