pub mod test;
use crate::backend::Buffer;

pub(crate) enum RenderPass {
    ClearColor {
        color: [f64; 4],
    },

    DrawIndexed {
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        pipeline: wgpu::RenderPipeline,
    },
}
