pub mod test;
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
    },
}
