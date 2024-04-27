use crate::backend::Buffer;

use std::sync::Arc;

pub enum RenderPass<'a> {
    ClearColor {
        color: [f64; 4],
    },

    DrawIndexed {
        vertex_buffer: &'a Buffer,
        index_buffer: &'a Buffer,
        pipeline: &'a wgpu::RenderPipeline,
        bind_groups : &'a Vec<&'a wgpu::BindGroup>
    },
    DrawInstanced {
        vertex_buffer: &'a Buffer,
        index_buffer: &'a Buffer,
        instance_buffer: &'a Buffer,
        instance_range: std::ops::Range<u32>,
        pipeline: &'a wgpu::RenderPipeline,
        bind_groups : &'a Vec<&'a wgpu::BindGroup>
    },
}
